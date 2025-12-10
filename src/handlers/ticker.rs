use crate::models::Ticker;
use crate::templates::TickersTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn tickers(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all tickers

    // time start
    let start = Instant::now();

    let tickers = sqlx::query_as::<_, Ticker>(
        "SELECT exchange, symbol, symbol_name, taker_fee_rate, 
                maker_fee_rate, taker_coefficient, maker_coefficient 
            FROM ticker",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let tickers_with_index: Vec<(usize, Ticker)> = tickers
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template = TickersTemplate {
        tickers: tickers_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
