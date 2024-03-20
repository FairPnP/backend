use alcoholic_jwt::ValidJWT;
use async_trait::async_trait;
use log::info;

use nanoid::nanoid;
use pingora::upstreams::peer::HttpPeer;

use pingora::Result;

use pingora::http::{RequestHeader, ResponseHeader};
use pingora::proxy::{ProxyHttp, Session};

use crate::auth::{jwt::validate_token, state::JwtValidatorState};
use crate::route_map::RouteMap;

pub struct MyGateway {
    name: String,
    req_metric: prometheus::IntCounter,
    jwt_validator: JwtValidatorState,
    route_map: RouteMap,
}

impl MyGateway {
    pub fn new(name: String, jwt_validator: JwtValidatorState, route_map: RouteMap) -> Self {
        Self {
            name,
            req_metric: prometheus::register_int_counter!("reg_counter", "Number of requests")
                .unwrap(),
            jwt_validator,
            route_map,
        }
    }

    pub async fn check_login(&self, req: &RequestHeader) -> Option<ValidJWT> {
        let authorization = req.headers.get("Authorization");
        // Bearer token
        if let Some(auth) = authorization {
            let auth = match auth.to_str() {
                Ok(auth) => auth,
                Err(_) => return None,
            };

            let split = auth.split_whitespace().collect::<Vec<&str>>();
            if split.len() == 2 && split[0] == "Bearer" {
                let token = split[1];
                match validate_token(
                    token,
                    self.jwt_validator.get_issuer(),
                    self.jwt_validator.get_jwks().await,
                ) {
                    Ok(jwt) => return Some(jwt),
                    Err(_) => return None,
                }
            }
        }

        None
    }
}

#[async_trait]
impl ProxyHttp for MyGateway {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        if session.req_header().uri.path() == "/health" {
            return Ok(false);
        }

        let jwt = Self::check_login(self, session.req_header()).await;
        if let Some(jwt) = jwt {
            let _ = session
                .req_header_mut()
                .insert_header("X-Auth-User", jwt.claims["sub"].to_string());
        } else {
            let _ = session.respond_error(401).await;
            // true: early return as the response is already written
            return Ok(true);
        }

        // false: continue to next filter
        return Ok(false);
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let request_id = nanoid!();
        info!(
            "path: {path}, request_id: {request_id}",
            path = session.req_header().uri.path(),
            request_id = request_id
        );

        let route = self.route_map.find_route(session.req_header().uri.path());
        if let Some((route, _)) = route {
            // add request id to the header
            session
                .req_header_mut()
                .insert_header("X-Request-ID", request_id)?;

            let peer = Box::new(HttpPeer::new(route.base_url.clone(), false, String::new()));
            return Ok(peer);
        }

        // 404
        Err(pingora::Error::new(pingora::ErrorType::HTTPStatus(404)))
    }

    async fn response_filter(
        &self,
        _session: &mut Session,

        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        // replace existing header if any
        upstream_response
            .insert_header("Server", &self.name)
            .unwrap();

        // because we don't support h3
        upstream_response.remove_header("alt-svc");

        Ok(())
    }

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        info!(
            "{} response code: {response_code}",
            self.request_summary(session, ctx)
        );

        self.req_metric.inc();
    }
}
