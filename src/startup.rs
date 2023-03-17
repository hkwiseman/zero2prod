use axum::{routing::get, routing::post, Router, Server};
use sqlx::PgPool;
use tower_http::{trace::{TraceLayer, DefaultOnResponse}, LatencyUnit};
use std::{net::TcpListener, sync::Arc};
use tracing::Level;
pub struct AppState {
    pub connection: PgPool,
}
pub async fn run(listener: TcpListener, connection: PgPool) {
    tracing_subscriber::fmt::init();
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
        .layer(TraceLayer::new_for_http()
            .on_response(DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Millis))
            )
        .with_state(shared_state)
}
