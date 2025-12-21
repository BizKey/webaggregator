use crate::models::Symbol;
use crate::templates::SymbolsTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn tradeable(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // time start
    let start = Instant::now();

    let tradeable_symbol: Vec<Symbol> = sqlx::query_as::<_, Symbol>(
        "SELECT 
                exchange, symbol, symbol_name, base_currency, quote_currency, fee_currency, 
                market, base_min_size, quote_min_size, base_max_size, quote_max_size, 
                base_increment, quote_increment, price_increment, price_limit_rate, 
                min_funds, is_margin_enabled, enable_trading, fee_category, 
                maker_fee_coefficient, taker_fee_coefficient, st, updated_at
            FROM symbol WHERE is_margin_enabled = true AND enable_trading = true AND fee_category = 1 AND quote_currency = 'USDT' ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let tradeable_symbol_with_index: Vec<(usize, Symbol)> = tradeable_symbol
        .into_iter()
        .enumerate()
        .map(|(i, symbol)| (i + 1, symbol))
        .collect();

    let template = SymbolsTemplate {
        symbols: tradeable_symbol_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn symbols(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // time start
    let start = Instant::now();

    let symbols: Vec<Symbol> = sqlx::query_as::<_, Symbol>(
        "SELECT exchange, symbol, symbol_name, base_currency, quote_currency, 
        fee_currency, market, base_min_size, quote_min_size, base_max_size, 
        quote_max_size, base_increment, quote_increment, price_increment, price_limit_rate, 
        min_funds, is_margin_enabled, enable_trading, fee_category, maker_fee_coefficient, 
        taker_fee_coefficient, st, updated_at
            FROM symbol ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let symbols_with_index: Vec<(usize, Symbol)> = symbols
        .into_iter()
        .enumerate()
        .map(|(i, symbol)| (i + 1, symbol))
        .collect();

    let template = SymbolsTemplate {
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
