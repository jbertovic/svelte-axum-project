use std::sync::Arc;

use axum::{
    http::{self, Request, StatusCode},
    response::{Response, IntoResponse},
    middleware::Next, Json,
};
use axum_sessions::async_session::serde_json::json;

use crate::store::Store;

/// middleware function to authenticate authorization token
/// check store that contains token and see if it matches authorization header starting with "Bearer"
/// used example in axum docs on middleware https://docs.rs/axum/latest/axum/middleware/index.html
/// 
/// Returns Error's in JSON format.  This could be included in another layer to handle errors to output as JSON
pub async fn auth<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        tracing::info!("Authorization header missing");
        // return Err(StatusCode::UNAUTHORIZED);
        return Ok(Json(json!( {"error": "Unauthorized"} )).into_response());
    };

    tracing::info!("Received Authorization Header: {}", auth_header);

    // check bearer authorization to see if it matches
    if let Some(store) = req.extensions().get::<Arc<Store>>() {
        if store.api_token_check(auth_header) {
            Ok(next.run(req).await)
        } else {
            tracing::info!("Authorization token does NOT match");
            // Err(StatusCode::UNAUTHORIZED)
            return Ok(Json(json!( {"error": "Unauthorized"} )).into_response());
        }

    } else {
        tracing::debug!("Can't retrieve Store");
        // Err(StatusCode::INTERNAL_SERVER_ERROR)
        return Ok(Json(json!( {"error": "Internal Server Error"} )).into_response());
    }

}