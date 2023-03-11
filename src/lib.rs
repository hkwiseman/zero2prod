use axum::{routing::get, Router, Server};

async fn health_check() -> hyper::StatusCode {
    hyper::StatusCode::OK
}

pub async fn run(address: &str) {
    let app = router();

    let addr = address.parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

pub fn router() -> Router {
    Router::new().route("/health_check", get(health_check))
}
