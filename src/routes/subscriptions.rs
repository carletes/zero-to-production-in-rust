use std::convert::TryInto;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryInto<NewSubscriber> for FormData {
    type Error = String;

    fn try_into(self) -> Result<NewSubscriber, Self::Error> {
        let name = SubscriberName::parse(self.name)?;
        let email = SubscriberEmail::parse(self.email)?;
        Ok(NewSubscriber { name, email })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, db_pool),
    fields(
        email = %form.email,
        name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let subscriber = form
        .0
        .try_into()
        .map_err(|_| HttpResponse::BadRequest().finish())?;

    insert_subscriber(&db_pool, &subscriber)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Inserting new subscriber in the database",
    skip(subscriber, db_pool)
)]
async fn insert_subscriber(
    db_pool: &PgPool,
    subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Cannot insert subscriber in database: {:?}", e);
        e
    })?;
    Ok(())
}
