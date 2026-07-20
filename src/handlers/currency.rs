use crate::api::models::Currency;
use crate::api::templates::CurrenciesTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn currencies(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    // all currency

    // time start
    let start: Instant = Instant::now();

    let all_currencies: Vec<Currency> = match sqlx::query_as::<_, Currency>(
        "SELECT exchange, currency, currency_name, full_name, precision, is_margin_enabled, is_debit_enabled, updated_at FROM currency ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(all_currencies) => all_currencies,
        Err(e) => {
            let msg: String = format!("Database error: {}", e);
            log::error!("{}", msg);
            return Ok(actix_web::error::ErrorInternalServerError("Database error").into())
        }
    };

    let currencies_with_index: Vec<(usize, Currency)> = all_currencies
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let template: CurrenciesTemplate = CurrenciesTemplate {
        currencies: currencies_with_index,
        elapsed_ms,
    };

    let html: String = match template.render() {
        Ok(html) => html,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Error template render")),
    };

    let response: HttpResponse = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);

    Ok(response)
}
