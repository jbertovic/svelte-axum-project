use axum::{body::Body, http::Request, response::IntoResponse};

pub async fn not_implemented_route(req: Request<Body>) -> impl IntoResponse {
    // add which route is requesting this?
    format!("Route is planned but not yet implemented for {}", req.uri())
}
