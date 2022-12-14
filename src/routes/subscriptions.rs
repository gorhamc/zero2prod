use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
        );

    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber to database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            log::info!("request_id {} - New subscriber details '{}' '{}' have been saved", request_id, form.name, form.email);
            HttpResponse::Ok().finish()
            },
        Err(e) => {
            log::error!("request_id {} - failed to execute query {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
