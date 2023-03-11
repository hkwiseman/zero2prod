use axum::{routing::get, Router, Server};
use std::net::SocketAddr;

async fn health_check() -> hyper::StatusCode {
    hyper::StatusCode::OK
}

pub async fn run() {
    let app = router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

pub fn router() -> Router {
    Router::new().route("/health_check", get(health_check))
}
