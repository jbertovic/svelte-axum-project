// print out session

use axum::{Extension, response::IntoResponse};
use axum_sessions::async_session::Session;

pub async fn session_out_handler(Extension(session): Extension<Session>) -> impl IntoResponse {
    tracing::info!("Seeking session info");
    format!("{:?}", session)
}