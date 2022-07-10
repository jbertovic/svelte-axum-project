use axum::{response::IntoResponse, Json, Extension};
use axum_sessions::async_session::{serde_json::json, Session};
use serde::Deserialize;

/// route to handle log in
pub async fn login(Json(login): Json<Login>, Extension(mut session): Extension<Session>) -> impl IntoResponse {
    tracing::info!("Logging in user: {}", login.username);

    if check_password(&login.username, &login.password) {
        session.insert("user_id", login.username).unwrap();
        Json(json!({"result": "ok"}))
    } else {
        Json(json!({"result": "error"}))
    }

}

/// route to handle log out
pub async fn logout(Extension(mut session): Extension<Session>) -> impl IntoResponse {
    let user = session.get_raw("user_id").unwrap_or_default();
    tracing::info!("Logging out user: {}", user);
    // drop session
    session.destroy();
    Json(json!({"result": "ok"}))
}

// assume all passwords work
fn check_password(_username: &str, _password: &str) -> bool {
    true
}


#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}