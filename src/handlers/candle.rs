use crate::models::{Candle, CandleWithAtr, calculate_atr};
use crate::templates::{CandleTemplate, CandlesTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;
pub async fn candle(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all candle

    // time start
    let start = Instant::now();
    let symbol_name = path.into_inner();

    let mut candles = sqlx::query_as::<_, Candle>(
        "SELECT exchange, symbol, interval, timestamp, open, high, low, close, volume, quote_volume
            FROM candle 
            WHERE symbol = $1
            ORDER BY symbol, timestamp::BIGINT DESC",
    )
    .bind(&symbol_name)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    candles.reverse();

    let candles_with_atr = calculate_atr(&candles, 20);

    let mut processed_candles: Vec<CandleWithAtr> = candles_with_atr
        .into_iter()
        .map(|mut c| {
            if let Some(atr) = c.atr {
                let close: f64 = c.close.parse().unwrap_or(0.0);
                if close > 0.0 {
                    c.atr_percent = Some((atr / close) * 100.0);
                } else {
                    c.atr_percent = None;
                }
                c.atr = None; // Убираем обычное значение ATR если нужно
            }
            c
        })
        .collect::<Vec<CandleWithAtr>>();

    processed_candles.reverse();

    let template = CandleTemplate {
        candles: processed_candles,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn candles(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all candle

    // time start
    let start = Instant::now();

    let all_candles = sqlx::query_as::<_, Candle>(
        "SELECT DISTINCT ON (symbol) 
                exchange, symbol, interval, timestamp, open, high, low, close, volume, quote_volume 
            FROM candle 
            ORDER BY symbol, timestamp::BIGINT DESC",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let candles_with_index: Vec<(usize, Candle)> = all_candles
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template = CandlesTemplate {
        candles: candles_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
