use std::sync::Arc;

use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::store::Store;

/// middleware function to authenticate authorization token
/// check store that contains token and see if it matches authorization header starting with "Bearer"
/// used example in axum docs on middleware https://docs.rs/axum/latest/axum/middleware/index.html
///
/// Returns Error's in JSON format.  
pub async fn auth<B>(
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
    if let Some(store) = req.extensions().get::<Arc<Store>>() {
        if store.api_token_check(auth_header) {
            Ok(next.run(req).await)
        } else {
            tracing::debug!("Authorization token does NOT match");
            //            return Ok(Json(json!( {"error": "Unauthorized"} )).into_response());
            Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())))
        }
    } else {
        tracing::debug!("Can't retrieve Store");
        //        return Ok(Json(json!( {"error": "Internal Server Error"} )).into_response());
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(JsonError::internal()),
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub fn new(error: String) -> Self {
        JsonError { error }
    }

    pub fn unauthorized() -> Self {
        JsonError {
            error: "Unauthorized".into(),
        }
    }

    pub fn internal() -> Self {
        JsonError {
            error: "Internal Server Error".into(),
        }
    }
}
