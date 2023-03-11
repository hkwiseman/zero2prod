use axum_test_helper::TestClient;
use zero2prod::router;
#[tokio::test]
async fn health_check_works() {
    let test_client = spawn_app();

    let response = test_client.get("/health_check").send().await;

    assert!(response.status().is_success());
    assert_eq!(response.text().await, "");
}

fn spawn_app() -> TestClient {
    let router = router();
    let client = TestClient::new(router);

    client
}