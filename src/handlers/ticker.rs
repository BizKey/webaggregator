use crate::api::models::Ticker;
use crate::api::templates::TickersTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn tickers(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let tickers: Vec<Ticker> =  sqlx::query_as::<_, Ticker>(
        r#"
        SELECT exchange, symbol, symbol_name, taker_fee_rate, maker_fee_rate, taker_coefficient, maker_coefficient, updated_at
        FROM ticker
        ORDER BY updated_at DESC;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let tickers: Vec<(usize, Ticker)> = tickers
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

    let elapsed_ms: u128 = start.elapsed().as_millis();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            TickersTemplate {
                tickers,
                elapsed_ms,
            }
            .render()
            .map_err(|e| {
                log::error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
