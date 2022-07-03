
/// Server that is split into a frontend to serve static files (Svelte) and backend that
/// runs the api and manages sessions
/// 
/// TODO: test api from svelte....to see i can capture JSON... also understand if cors is needed
/// TODO: add sessions
/// TODO: login to set session
/// TODO: logout to reset session

use std::env;
use std::{io, net::SocketAddr};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use tower_http::{
    services::ServeDir,
    trace::TraceLayer
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum_sessions::{
    async_session::MemoryStore,
    SessionLayer,
};

pub mod middleware;
pub mod routes;

// figure out a way to modify the cookie name
const SESSION_COOKIE_NAME: &str = "axum_swelte_session";

#[tokio::main]
async fn main() {

    // start tracing
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
    let secret = env::var("SERVER_SECRET")
        .ok()
        .unwrap_or_else(|| "this needs to be 64bytes. recommended that you set Secret instead of fixed value".to_string());
    
        let addr: SocketAddr = format!("{}:{}", host, port).parse().expect("Can not parse address and port");

    // Front end to server svelte build bundle, css and index.html from public folder
    let frontend = Router::new()
        .fallback(get_service(ServeDir::new("./public")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());

    // setup up sessions and store to keep track of session information
    let session_layer = SessionLayer::new(MemoryStore::new(), secret.as_bytes()).with_cookie_name(SESSION_COOKIE_NAME);

    let backend = Router::new()
        .route(
           "/api", get(|| async { "/api not yet implemented"})
        )
        .route("/session", get(routes::session::session_out_handler ) )
        .layer(session_layer);

    let app = Router::new()
        .merge(frontend)
        .merge(backend);

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    }


async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
