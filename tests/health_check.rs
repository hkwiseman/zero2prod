use axum_test_helper::TestClient;
use zero2prod::{startup::router, configuration::get_configuration};
use sqlx::{PgConnection, Connection};

fn spawn_app() -> TestClient {
    let router = router();
    let client = TestClient::new(router);

    client
}

#[tokio::test]
async fn health_check_works() {
    let test_client = spawn_app();

    let response = test_client.get("/health_check").send().await;

    assert!(response.status().is_success());
    assert_eq!(response.text().await, "");
}

#[tokio::test]
async fn subscribe_returns_200() {
    let test_client = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");


    let body = "name=stan%20lee&email=excelsior123%40gmail.com";
    let response = test_client
        .post("/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "excelsior123@gmail.com");
    assert_eq!(saved.name, "stan lee");
}

#[tokio::test]
async fn invalid_form_subscribe_returns_400() {
    let test_client = spawn_app();

    let test_cases = vec![
        ("name=stan%20lee", "missing the email"),
        ("email=excelsior123%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_client
            .post("/subscriptions")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await;

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Unprocessable Entity when the payload was {}",
            error_message
        );
    }
}
