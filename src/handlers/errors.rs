use crate::api::models::Error;
use crate::api::templates::ErrorsTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn errors(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let errors: Vec<Error> = sqlx::query_as::<_, Error>(
        r#"
        SELECT exchange, msg, updated_at
        FROM errors
        ORDER BY updated_at
        DESC LIMIT 1000;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = ErrorsTemplate { errors, elapsed_ms }
        .render()
        .map_err(|e| {
            log::error!("Template render error: {}", e);
            actix_web::error::ErrorInternalServerError("Template render error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
