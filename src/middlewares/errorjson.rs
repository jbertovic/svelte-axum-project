use axum::{http::StatusCode, Json, response::IntoResponse};
use axum_sessions::async_session::serde_json::json;

pub async fn handle_error_to_json(err: StatusCode) -> impl IntoResponse 
{   
    tracing::info!("error Received and JSON sent");
    Json(json!( {"error": "Unauthorized"} ))
}