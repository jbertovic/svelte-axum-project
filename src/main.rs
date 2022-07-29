#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, get_service, post},
    Extension, Router,
};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use std::{env, sync::Arc};
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod store;

pub mod middlewares;
pub mod routes;

// figure out a way to modify the cookie name
const SESSION_COOKIE_NAME: &str = "axum_svelte_session";

/// Server that is split into a Frontend to serve static files (Svelte) and Backend
/// Backend is further split into a non authorized area and a secure area
/// The Back end is using 2 middleware: sessions (managing session data) and user_secure (checking for authorization)
#[tokio::main]
async fn main() {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "svelte_axum_project=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // configure server from environmental variables
    let port = env::var("SERVER_PORT")
        .ok()
        .unwrap_or_else(|| "8080".to_string());
    let host = env::var("SERVER_HOST")
        .ok()
        .unwrap_or_else(|| "127.0.0.1".to_string());
    let secret = env::var("SERVER_SECRET").ok().unwrap_or_else(|| {
        "this needs to be 64bytes. recommended that you set Secret instead of fixed value"
            .to_string()
    });

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Can not parse address and port");

    // create store for backend.  Stores an api_token.
    let shared_state = Arc::new(store::Store::new("123456789"));

    // Front end to server svelte build bundle, css and index.html from public folder
    let frontend = Router::new()
        .fallback(get_service(ServeDir::new("./public")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());

    // setup up sessions and store to keep track of session information
    let session_layer = SessionLayer::new(MemoryStore::new(), secret.as_bytes())
        .with_cookie_name(SESSION_COOKIE_NAME);

    // Back end to serve:
    // NON-AUTH AREA routes: login, logout and session
    // `/test` shows example of a non implemented route
    let non_auth_backend = Router::new()
        .route("/auth/session", get(routes::data_handler)) // gets session data
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::logout)) // deletes username in session
        .route("/test", get(routes::not_implemented_route));

    // AUTH AREA routes:
    // `/secure` shows an example of checking session information for user_id to allow access
    // `/api` can be accessed using an authorization header and with no session
    let auth_backend_using_token = Router::new()
        .route("/api", get(routes::handler))
        .route_layer(middleware::from_fn(middlewares::auth));
    let auth_backend_using_session = Router::new()
        .route("/secure", get(routes::out_handler))
        .route_layer(middleware::from_fn(middlewares::user_secure));

    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    let backend = Router::new()
        .merge(non_auth_backend)
        .merge(auth_backend_using_token)
        .merge(auth_backend_using_session)
        .layer(session_layer)
        .layer(Extension(shared_state));

    let app = Router::new().merge(frontend).merge(backend);

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[allow(clippy::unused_async)]
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
