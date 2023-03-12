use zero2prod::startup::run;
use zero2prod::configuration::{get_configuration};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let addr = format!("127.0.0.1:{}", configuration.application_port);
    run(&addr).await
}
