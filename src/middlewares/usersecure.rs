use axum::{
    http::{Request, StatusCode},
    response::Response,
    middleware::Next,
};
use axum_sessions::async_session::Session;

pub async fn user_secure<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    tracing::info!("Middleware: checking if user exists");
    let session = req.extensions()
        .get::<Session>().ok_or(StatusCode::UNAUTHORIZED)?;
    tracing::debug!("Session Extracted: {:?}", session);    
    let user_id = session.get_raw("user_id").ok_or(StatusCode::UNAUTHORIZED)?;
    tracing::debug!("user_id Extracted: {}", user_id);    
    Ok(next.run(req).await)
}