use std::sync::Arc;

use axum::{extract::State, Form};
use chrono::Utc;
use serde;
use uuid::Uuid;

use crate::startup::AppState;

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    Form(subscribe_user): Form<SubscribeForm>,
) -> hyper::StatusCode {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscribe_user.email,
        subscribe_user.name,
        Utc::now()
    )
    .execute(&state.connection)
    .await
    .expect("Failed to create user.");
    hyper::StatusCode::OK
}
