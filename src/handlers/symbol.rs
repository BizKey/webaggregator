use crate::api::models::Symbol;
use crate::api::templates::SymbolsTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn tradeable(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let tradeable_symbol =  sqlx::query_as::<_, Symbol>(
        "SELECT 
                exchange, symbol, symbol_name, base_currency, quote_currency, fee_currency, 
                market, base_min_size, quote_min_size, base_max_size, quote_max_size, 
                base_increment, quote_increment, price_increment, price_limit_rate, 
                min_funds, is_margin_enabled, enable_trading, fee_category, 
                maker_fee_coefficient, taker_fee_coefficient, st, updated_at
            FROM symbol WHERE is_margin_enabled = true AND enable_trading = true AND fee_category = 1 AND quote_currency = 'USDT' AND base_currency <> 'USDC' AND base_currency <> 'KCS' ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await.map_err(|e|{
        log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let tradeable_symbol_with_index: Vec<(usize, Symbol)> = tradeable_symbol
        .into_iter()
        .enumerate()
        .map(|(i, symbol)| (i + 1, symbol))
        .collect();

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let template: SymbolsTemplate = SymbolsTemplate {
        symbols: tradeable_symbol_with_index,
        elapsed_ms,
    };

    let html: String = template.render().map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn symbols(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

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
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let symbols_with_index: Vec<(usize, Symbol)> = symbols
        .into_iter()
        .enumerate()
        .map(|(i, symbol)| (i + 1, symbol))
        .collect();

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = SymbolsTemplate {
        symbols: symbols_with_index,
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
