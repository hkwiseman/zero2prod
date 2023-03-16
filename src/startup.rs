use axum::{routing::get, routing::post, Router, Server};
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};

pub struct AppState {
    pub connection: PgPool,
}
pub async fn run(listener: TcpListener, connection: PgPool) {
    let app = router(connection);

    Server::from_tcp(listener)
        .expect("Server failed to start.")
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
