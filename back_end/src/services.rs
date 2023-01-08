use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, get_service, post},
    Router,
};
use axum_sessions::{async_session::SessionStore, SessionLayer};
use std::{io, sync::Arc};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{middlewares, routes, store::{self, Store}, FRONT_PUBLIC};

// *********
// FRONT END
// *********
// Front end to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    //let serve_dir = get_service(ServeDir::new(FRONT_PUBLIC)).handle_error(handle_error);
    Router::new()
        .fallback_service(get_service(ServeDir::new(FRONT_PUBLIC)).handle_error(handle_error))
        .layer(TraceLayer::new_for_http())
}

#[allow(clippy::unused_async)]
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}

// ********
// BACK END
// ********
// Back end server built form various routes that are either public, require auth, or secure login
pub fn backend<Store: SessionStore>(
    session_layer: SessionLayer<Store>,
    shared_state: Arc<store::Store>,
) -> Router
{
    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    Router::new()
        .merge(back_public_route())
        .merge(back_auth_route())
        .merge(back_token_route(shared_state))
        .layer(session_layer)
}

// *********
// BACKEND NON-AUTH
// *********
//
pub fn back_public_route() -> Router {
    Router::new()
        .route("/auth/session", get(routes::session::data_handler)) // gets session data
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::logout)) // deletes username in session
        .route("/test", get(routes::not_implemented_route))
}

// *********
// BACKEND SESSION
// *********
//
pub fn back_auth_route() -> Router {
    Router::new()
        .route("/secure", get(routes::session::handler))
        .route_layer(middleware::from_fn(middlewares::user_secure))
}

// *********
// BACKEND API
// *********
//
// invoked with State that stores API that is checked by the `middleware::auth`
pub fn back_token_route<S>(state: Arc<Store>) -> Router<S> {
    Router::new()
        .route("/api", get(routes::api::handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), middlewares::auth))
        .with_state(state)
}
