use std::sync::Arc;

use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::store::Store;

/// middleware function to authenticate authorization token
/// check store that contains token and see if it matches authorization header starting with "Bearer"
/// used example in axum docs on middleware <https://docs.rs/axum/latest/axum/middleware/index.html>
///
/// Returns Error's in JSON format.  
#[allow(clippy::missing_errors_doc)]
pub async fn auth<B: Send + Sync>(
    State(store): State<Arc<Store>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, Json<JsonError>)> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        tracing::debug!("Authorization header missing");
        return Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())));
    };

    tracing::debug!("Received Authorization Header: {}", auth_header);

    // check bearer authorization to see if it matches
    if store.api_token_check(auth_header) {
        Ok(next.run(req).await)
    } else {
        tracing::debug!("Authorization token does NOT match");
        //            return Ok(Json(json!( {"error": "Unauthorized"} )).into_response());
        Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())))
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub const fn new(error: String) -> Self {
        Self { error }
    }

    pub fn unauthorized() -> Self {
        Self {
            error: "Unauthorized".into(),
        }
    }

    pub fn internal() -> Self {
        Self {
            error: "Internal Server Error".into(),
        }
    }
}
