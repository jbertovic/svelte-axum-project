// print out session

use axum::{response::IntoResponse, Json};
use serde_json::json;
use tower_sessions::Session;

/// output entire session object
#[allow(clippy::unused_async)]
pub async fn handler(session: Session) -> impl IntoResponse {
    tracing::info!("Seeking session info");
    Json(json!({ "session": format!("{:?}", session) }))
}

/// output session data in json
#[allow(clippy::unused_async)]
pub async fn data_handler(session: Session) -> impl IntoResponse {
    tracing::info!("Seeking session data");
    let user_id = session.get_value("user_id").unwrap_or_else(|| "".into());
    Json(json!({ "user_id": user_id }))
}
