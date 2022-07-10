use axum::{
    http::{self, Request, StatusCode},
    response::Response,
    middleware::Next,
};

use crate::store::Store;

// middleware function to authenticate authorization token

// check store that contains token and see if it matches authorization header starting with "Bearer"

// used example in axum docs on middleware https://docs.rs/axum/latest/axum/middleware/index.html
pub async fn auth<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        tracing::info!("Authorization header missing");
        return Err(StatusCode::UNAUTHORIZED);
    };

    tracing::info!("Received Authorization Header: {}", auth_header);

    // check bearer authorization to see if it matches
    if let Some(store) = req.extensions().get::<Store>() {
        if store.api_token_check(auth_header) {
            Ok(next.run(req).await)
        } else {
            tracing::info!("Authorization token does NOT match");
            Err(StatusCode::UNAUTHORIZED)
        }

    } else {
        tracing::debug!("Can't retrieve Store");
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}