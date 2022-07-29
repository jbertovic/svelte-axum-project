// print out session

use axum::{response::IntoResponse, Extension, Json};
use axum_sessions::async_session::{serde_json::json, Session};

/// output entire session object
#[allow(clippy::unused_async)]
pub async fn out_handler(Extension(session): Extension<Session>) -> impl IntoResponse {
    tracing::info!("Seeking session info");
    Json(json!({ "session": format!("{:?}", session) }))
}

/// output session data in json
#[allow(clippy::unused_async)]
pub async fn data_handler(Extension(session): Extension<Session>) -> impl IntoResponse {
    tracing::info!("Seeking session data");
    let user_id = session.get("user_id").unwrap_or_else(|| "".to_string());
    Json(json!({ "user_id": user_id }))
}
