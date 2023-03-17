use tracing::info;
pub async fn health_check() -> hyper::StatusCode {
    info!("Healthy check!");
    hyper::StatusCode::OK
}
