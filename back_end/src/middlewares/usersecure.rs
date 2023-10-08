use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_sessions::Session;

#[allow(clippy::missing_errors_doc)]
pub async fn user_secure<B: Send>(
    session: Session,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    tracing::info!("Middleware: checking if user exists");
    let user_id = session
        .get_value("user_id")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    tracing::debug!("user_id Extracted: {}", user_id);

    // accepts all user but you could add a check here to match user access
    Ok(next.run(req).await)
}
