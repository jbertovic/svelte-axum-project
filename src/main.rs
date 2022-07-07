use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post},
    Router,
    middleware,
};
use std::env;
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum_sessions::{async_session::MemoryStore, SessionLayer};

pub mod middlewares;
pub mod routes;


// figure out a way to modify the cookie name
const SESSION_COOKIE_NAME: &str = "axum_swelte_session";

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

    // Front end to server svelte build bundle, css and index.html from public folder
    let frontend = Router::new()
        .fallback(get_service(ServeDir::new("./public")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());

    // setup up sessions and store to keep track of session information
    let session_layer = SessionLayer::new(MemoryStore::new(), secret.as_bytes())
        .with_cookie_name(SESSION_COOKIE_NAME);

    // Back end to serve:
    // NON-AUTH AREA routes: login, logout and session
    let non_auth_backend = Router::new()
        .route("/auth/session", get(routes::session_data_handler)) // gets session data
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::not_implemented_route)) // deletes username in session
        .route("/test", get(routes::not_implemented_route));

    // AUTH AREA routes: secure
    let auth_backend = Router::new()
        .route("/secure", get(routes::session_out_handler))
        .route_layer(middleware::from_fn(middlewares::user_secure));

    let backend = Router::new()
        .merge(non_auth_backend)
        .merge(auth_backend)
        .layer(session_layer);

    let app = Router::new().merge(frontend).merge(backend);

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong accessing static files...")
}
