use axum::{routing::get, routing::post, Router, Server};

pub async fn run(address: &str) {
    let app = router();

    let addr = address.parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

pub fn router() -> Router {
    Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .route("/subscriptions", post(crate::routes::subscribe))
}
