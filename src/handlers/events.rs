use crate::models::Event;
use crate::templates::EventsTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;
pub async fn events(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // events

    // time start
    let start = Instant::now();

    let events = sqlx::query_as::<_, Event>(
        "SELECT exchange, msg, updated_at FROM events ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let template = EventsTemplate {
        events: events,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
