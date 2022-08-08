use axum::{
    extract::RequestParts,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_sessions::{extractors::ReadableSession};

#[allow(clippy::missing_errors_doc)]
pub async fn user_secure<B: Send>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    tracing::info!("Middleware: checking if user exists");
    let mut request_parts = RequestParts::new(req);
    let session = request_parts
        .extract::<ReadableSession>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let user_id = session.get_raw("user_id").ok_or(StatusCode::UNAUTHORIZED)?;
    tracing::debug!("user_id Extracted: {}", user_id);

    // accepts all user but you could add a check here to match user access

    let req = request_parts.try_into_request().expect("body extracted");
    Ok(next.run(req).await)
}
