use crate::route_map::RouteMap;

pub fn get_route_map() -> RouteMap {
    let mut route_map = RouteMap::new();

    let api_url = std::env::var("API_URL").expect("API_URL is not set");
    let stripe_url = std::env::var("STRIPE_URL").expect("STRIPE_URL is not set");
    let stripe_webhook_url =
        std::env::var("STRIPE_WEBHOOK_URL").expect("STRIPE_WEBHOOK_URL is not set");

    // health
    // TODO: include all services
    route_map.add_route(&api_url, "/health", vec!["GET"]);

    // stripe webhook
    // TODO: move out of gateway
    route_map.add_route(&stripe_webhook_url, "/webhooks", vec!["POST"]);

    // availability
    route_map.add_route(&api_url, "/api/availability/v1", vec!["GET", "POST"]);
    route_map.add_route(&api_url, "/api/availability/v1/search", vec!["POST"]);
    route_map.add_route(
        &api_url,
        "/api/availability/v1/{id}",
        vec!["GET", "PUT", "DELETE"],
    );

    // buildings
    route_map.add_route(&api_url, "/api/buildings/v1", vec!["GET"]);
    route_map.add_route(&api_url, "/api/buildings/v1/{id}", vec!["GET"]);

    // chat
    route_map.add_route(&api_url, "/api/chat/v1/host", vec!["GET"]);
    route_map.add_route(&api_url, "/api/chat/v1/guest", vec!["GET"]);
    route_map.add_route(&api_url, "/api/chat/v1/{reservation_id}", vec!["GET"]);
    route_map.add_route(&api_url, "/api/chat/v1", vec!["POST"]);

    // dev
    route_map.add_route(&api_url, "/api/dev/v1/reset-database/{db}", vec!["POST"]);

    // reservations
    route_map.add_route(&api_url, "/api/reservations/v1", vec!["GET", "POST"]);
    route_map.add_route(&api_url, "/api/reservations/v1/host", vec!["GET"]);
    route_map.add_route(
        &api_url,
        "/api/reservations/v1/{id}",
        vec!["GET", "PUT", "DELETE"],
    );

    // spaces
    route_map.add_route(&api_url, "/api/spaces/v1", vec!["GET", "POST"]);
    route_map.add_route(
        &api_url,
        "/api/spaces/v1/{id}",
        vec!["GET", "PUT", "DELETE"],
    );

    // space images
    route_map.add_route(&api_url, "/api/space_images/v1", vec!["GET"]);
    route_map.add_route(&api_url, "/api/space_images/v1/images", vec!["POST"]);
    route_map.add_route(
        &api_url,
        "/api/space_images/v1/images/complete",
        vec!["PUT"],
    );
    route_map.add_route(&api_url, "/api/space_images/v1/{id}", vec!["GET", "DELETE"]);

    // space reviews
    route_map.add_route(&api_url, "/api/space_reviews/v1", vec!["GET", "POST"]);
    route_map.add_route(
        &api_url,
        "/api/space_reviews/v1/{id}",
        vec!["GET", "PUT", "DELETE"],
    );

    // space summaries
    route_map.add_route(&api_url, "/api/space_summaries/v1/{id}", vec!["GET"]);

    // stripe accounts
    route_map.add_route(&stripe_url, "/api/accounts/v1", vec!["GET"]);
    route_map.add_route(&stripe_url, "/api/accounts/v1/dashboard", vec!["POST"]);

    // stripe customers
    route_map.add_route(&stripe_url, "/api/customers/v1", vec!["GET"]);
    route_map.add_route(
        &stripe_url,
        "/api/customers/v1/payment_intent",
        vec!["POST"],
    );

    // stripe events
    route_map.add_route(&stripe_url, "/api/events/v1", vec!["GET"]);

    // user notifs
    route_map.add_route(&api_url, "/api/user_notifs/v1/token", vec!["GET", "POST"]);

    // user profiles
    route_map.add_route(&api_url, "/api/user_profiles/v1", vec!["PUT"]);
    route_map.add_route(&api_url, "/api/user_profiles/v1/{id}", vec!["GET"]);
    route_map.add_route(&api_url, "/api/user_profiles/v1/avatar", vec!["POST"]);

    // user reviews
    route_map.add_route(&api_url, "/api/user_reviews/v1", vec!["GET", "POST"]);
    route_map.add_route(
        &api_url,
        "/api/user_reviews/v1/{id}",
        vec!["GET", "PUT", "DELETE"],
    );

    // user summaries
    route_map.add_route(&api_url, "/api/user_summaries/v1/{id}", vec!["GET"]);

    route_map
}
