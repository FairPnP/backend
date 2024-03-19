// Copyright 2024 Cloudflare, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and

// limitations under the License.

use std::env;

use gateway::MyGateway;
use structopt::StructOpt;

use pingora::server::configuration::Opt;
use pingora::server::Server;

mod auth;
mod gateway;

fn main() {
    env_logger::init();

    // read command line arguments
    let opt = Opt::from_args();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let issuer = env::var("AUTH_ISSUER").expect("AUTH_ISSUER is not set");
    let jwks_uri = env::var("AUTH_JWKS_URI").expect("AUTH_JWKS_URI is not set");
    let jwt_validator = auth::state::JwtValidatorState::new(issuer, jwks_uri);

    // block until jwks is fetched jwt_validator.get_jwks();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(jwt_validator.get_jwks());

    let server_name = env::var("SERVER_NAME").expect("SERVER_NAME is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let mut my_proxy = pingora::proxy::http_proxy_service(
        &my_server.configuration,
        MyGateway::new(server_name, jwt_validator),
    );
    my_proxy.add_tcp(&format!("0.0.0.0:{}", port));
    my_server.add_service(my_proxy);

    let prometheus_port = env::var("PROMETHEUS_PORT").expect("PROMETHEUS_PORT is not set");
    let mut prometheus_service_http =
        pingora::services::listening::Service::prometheus_http_service();
    prometheus_service_http.add_tcp(&format!("0.0.0.0:{}", prometheus_port));
    my_server.add_service(prometheus_service_http);

    my_server.run_forever();
}
