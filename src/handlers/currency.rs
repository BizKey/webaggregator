use crate::models::Currency;
use crate::templates::CurrenciesTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn currencies(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all currency

    // time start
    let start = Instant::now();

    let all_currencies = sqlx::query_as::<_, Currency>(
        "SELECT exchange, currency, currency_name, full_name, is_margin_enabled, is_debit_enabled, updated_at FROM currency ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let currencies_with_index: Vec<(usize, Currency)> = all_currencies
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = CurrenciesTemplate {
        currencies: currencies_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
