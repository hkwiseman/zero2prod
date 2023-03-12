use axum::Form;
use serde;

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    email: String,
    name: String,
}

pub async fn subscribe(Form(_subscribe_user): Form<SubscribeForm>) -> hyper::StatusCode {
    hyper::StatusCode::OK
}
