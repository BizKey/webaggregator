use crate::models::{CandleForSma, CandleForSmaSymbol};
use crate::templates::{CandlesSmaSymbolTemplate, CandlesSmaTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn smastrategy_by_symbol(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    // sma strategy by symbol
    let symbol_name = path.into_inner();
    // time start
    let start = Instant::now();

    let all_symbols = sqlx::query_as::<_, CandleForSmaSymbol>(
        "SELECT close 
            FROM (
                SELECT close, timestamp::BIGINT 
                FROM candle 
                WHERE symbol = $1 
                ORDER BY timestamp::BIGINT DESC 
                LIMIT 1000
            ) AS latest 
        ORDER BY timestamp ASC;",
    )
    .bind(&symbol_name)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let symbols_with_index: Vec<(usize, CandleForSmaSymbol)> = all_symbols
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template = CandlesSmaSymbolTemplate {
        symbols: symbols_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn smastrategy(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // sma strategy

    // time start
    let start = Instant::now();

    let all_symbols =
        sqlx::query_as::<_, CandleForSma>("SELECT symbol FROM candle GROUP BY symbol")
            .fetch_all(pool.get_ref())
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?;

    let symbols_with_index: Vec<(usize, CandleForSma)> = all_symbols
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template = CandlesSmaTemplate {
        symbols: symbols_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
