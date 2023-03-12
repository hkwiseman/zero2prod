use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let addr = format!("127.0.0.1:{}", configuration.application_port);
    run(&addr, connection).await
}
