use crate::api::models::Currency;
use crate::api::templates::CurrenciesTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn currencies(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let all_currencies: Vec<Currency> = sqlx::query_as::<_, Currency>(
        "SELECT exchange, currency, currency_name, full_name, precision, is_margin_enabled, is_debit_enabled, updated_at FROM currency ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let currencies_with_index: Vec<(usize, Currency)> = all_currencies
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = CurrenciesTemplate {
        currencies: currencies_with_index,
        elapsed_ms,
    }
    .render()
    .map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
