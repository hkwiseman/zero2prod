use crate::startup::AppState;
use axum::{extract::State, Form};
use chrono::Utc;
use serde;
use sqlx::{PgPool, postgres::PgQueryResult};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    pub email: String,
    pub name: String,
}
#[tracing::instrument(
    name = "saving new subscriber into the database",
    skip(form, pool)
    fields(
        id = %request_id
    )
)]
async fn insert_subscriber(
    pool: &PgPool,
    form: &SubscribeForm,
    request_id: Uuid
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    
}

#[tracing::instrument(
    name = "Passing data to insert functionality",
    skip(subscribe_user, state),
    fields(
        email = %subscribe_user.email,
        name = %subscribe_user.name,
    )
)]
pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    Form(subscribe_user): Form<SubscribeForm>,
) -> hyper::StatusCode {
    let request_id = Uuid::new_v4();
    info!("request_id {} - Adding '{}' '{}' as a new subscriber.", request_id, subscribe_user.name, subscribe_user.email);
    let result = insert_subscriber(&state.connection, &subscribe_user, request_id).await;
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
