
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

#[tokio::main]
async fn main() {

    // start tracing
    tracing_subscriber::fmt::init();

    // configure server from environmental variables
    let port = env::var("SERVER_PORT")
        .ok()
        .unwrap_or_else(|| "8080".to_string());
    let host = env::var("SERVER_HOST")
        .ok()
        .unwrap_or_else(|| "127.0.0.1".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().expect("Can not parse address and port");

    let frontend = Router::new()
        .fallback(get_service(ServeDir::new("./public")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());


    let backend = Router::new()
        .route(
           "/api", get(|| async { "/api not yet implemented"})
        );

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
