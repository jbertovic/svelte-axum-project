use axum::{response::IntoResponse, Json};
use serde_json::json;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn handler() -> impl IntoResponse {
    tracing::info!("Seeking api data");
    Json(
        json!({"result": "ok", "message": "You've reached the backend API by using a valid token."}),
    )
}
