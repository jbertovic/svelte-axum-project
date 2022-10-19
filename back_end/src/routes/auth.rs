use axum::{response::IntoResponse, Json};
use axum_sessions::{async_session::serde_json::json, extractors::WritableSession};
use serde::Deserialize;

/// route to handle log in
#[allow(clippy::unused_async)]
#[allow(clippy::missing_panics_doc)]
pub async fn login(Json(login): Json<Login>, mut session: WritableSession) -> impl IntoResponse {
    tracing::info!("Logging in user: {}", login.username);

    if check_password(&login.username, &login.password) {
        session.insert("user_id", login.username).unwrap();
        Json(json!({"result": "ok"}))
    } else {
        Json(json!({"result": "error"}))
    }
}

/// route to handle log out
#[allow(clippy::unused_async)]
pub async fn logout(mut session: WritableSession) -> impl IntoResponse {
    let user = session.get_raw("user_id").unwrap_or_default();
    tracing::info!("Logging out user: {}", user);
    // drop session
    session.destroy();
    Json(json!({"result": "ok"}))
}

// assume all passwords work
const fn check_password(_username: &str, _password: &str) -> bool {
    true
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}
