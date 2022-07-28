use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_sessions::async_session::Session;

pub async fn user_secure<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    tracing::info!("Middleware: checking if user exists");
    let session = req
        .extensions()
        .get::<Session>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id = session.get_raw("user_id").ok_or(StatusCode::UNAUTHORIZED)?;
    tracing::debug!("user_id Extracted: {}", user_id);

    // accepts all user but you could add a check here to match user access

    Ok(next.run(req).await)
}
