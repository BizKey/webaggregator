use crate::api::models::Currency;
use crate::api::templates::CurrenciesTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn currencies(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let currencies: Vec<Currency> = sqlx::query_as::<_, Currency>(
        r#"
        SELECT exchange, currency, currency_name, full_name, precision, is_margin_enabled, is_debit_enabled, updated_at
        FROM currency
        ORDER BY updated_at DESC;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let currencies: Vec<(usize, Currency)> = currencies
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

    let elapsed_ms: u128 = start.elapsed().as_millis();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            CurrenciesTemplate {
                currencies,
                elapsed_ms,
            }
            .render()
            .map_err(|e| {
                log::error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
