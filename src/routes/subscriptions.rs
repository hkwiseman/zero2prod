use crate::startup::AppState;
use axum::{extract::State, Form};
use chrono::Utc;
use serde;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Saving new subscriber details into database",
    skip(subscribe_user, state)
)]
pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    Form(subscribe_user): Form<SubscribeForm>,
) -> hyper::StatusCode {
    let request_id = Uuid::new_v4();
    info!("request_id {} - Adding '{}' '{}' as a new subscriber.", request_id, subscribe_user.name, subscribe_user.email);
    let result = sqlx::query!(
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
    .await;
    match result {
        Ok(_) => {
            info!("request_id {} - Saved new subscriber details!", request_id);
            hyper::StatusCode::OK
        }
        Err(e) => {
            error!("request_id {} - Failed to execute query: {:?}",request_id, e);
            hyper::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
