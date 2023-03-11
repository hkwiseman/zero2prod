use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn health_check() -> hyper::StatusCode {
    hyper::StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health_check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server.");
}
