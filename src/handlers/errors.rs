use crate::api::models::Error;
use crate::api::templates::ErrorsTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;
pub async fn errors(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // errors

    // time start
    let start: Instant = Instant::now();

    let errors: Vec<Error> = match sqlx::query_as::<_, Error>(
        "SELECT exchange, msg, updated_at FROM errors ORDER BY updated_at DESC LIMIT 1000;",
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(errors) => errors,
        Err(e) => {
            let msg: String = format!("Database error: {}", e);
            log::error!("{}", msg);
            return Ok(actix_web::error::ErrorInternalServerError("Database error").into());
        }
    };

    let template: ErrorsTemplate = ErrorsTemplate {
        errors: errors,
        elapsed_ms: start.elapsed().as_millis(),
    };

    let html: String = match template.render() {
        Ok(html) => html,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Error template render")),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
