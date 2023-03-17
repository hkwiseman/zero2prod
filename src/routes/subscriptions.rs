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
    match sqlx::query!(
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
        .await {
        Ok(_) => {
            tracing::info_span!("New subscriber info saved.");
            hyper::StatusCode::OK
        },
        Err(e) => {
            let _error_message = String::from(format!("Failed to execute query: {:?}", e));
            hyper::StatusCode::INTERNAL_SERVER_ERROR
        },
    }


}
