// print out session

use axum::{response::IntoResponse, Extension, Json};
use axum_sessions::async_session::{Session, serde_json::json};

/// output entire session object
pub async fn session_out_handler(Extension(session): Extension<Session>) -> impl IntoResponse {
    tracing::info!("Seeking session info");
    Json(json!({"session": format!("{:?}",session)}))
}

/// output session data in json
pub async fn session_data_handler(Extension(session): Extension<Session>) -> impl IntoResponse {
    tracing::info!("Seeking session data");
    let user_id = session.get("user_id").unwrap_or("".to_string());
    Json(json!({"user_id": user_id}))
}

