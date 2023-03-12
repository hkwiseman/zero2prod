use axum::{routing::get, routing::post, Router, Server};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub connection: PgPool,
}
pub async fn run(address: &str, connection: PgPool) {
    let app = router(connection);

    let addr = address.parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

pub fn router(conn: PgPool) -> Router {
    let shared_state = Arc::new(AppState { connection: conn });
    Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .route("/subscriptions", post(crate::routes::subscribe))
        .with_state(shared_state)
}
