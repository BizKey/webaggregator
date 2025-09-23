use crate::models::{Borrow, Currency, Lend, Symbol, Ticker};
use crate::templates::{
    BorrowTemplate, CurrenciesTemplate, CurrencyTemplate, IndexTemplate, LendTemplate,
    SymbolTemplate, SymbolsTemplate, TickerTemplate, TickersTemplate,
};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn index() -> HttpResponse {
    let template = IndexTemplate {};

    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::InternalServerError().body("Error template render"),
    }
}

pub async fn symbols(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // time start
    let start = Instant::now();

    let symbols = sqlx::query_as::<_, Symbol>("SELECT DISTINCT ON (symbol) created_at, symbol, name, base_currency, quote_currency, fee_currency, market, base_min_size, quote_min_size, base_max_size, quote_max_size, base_increment, quote_increment, price_increment, price_limit_rate, min_funds, is_margin_enabled, enable_trading, fee_category, maker_fee_coefficient, taker_fee_coefficient, st, callauction_is_enabled, callauction_price_floor, callauction_price_ceiling, callauction_first_stage_start_time, callauction_second_stage_start_time, callauction_third_stage_start_time, trading_start_time FROM Symbol ORDER BY symbol, created_at DESC")
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

pub async fn symbol(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // time start
    let start = Instant::now();

    let symbols = sqlx::query_as::<_, Symbol>("SELECT created_at FROM Symbol")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let template = SymbolTemplate {
        symbols: symbols,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn currencies(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all currency

    // time start
    let start = Instant::now();

    let all_currencies = sqlx::query_as::<_, Currency>("SELECT DISTINCT ON (currency) created_at, currency, name, full_name, precision, confirms, contract_address, is_margin_enabled, is_debit_enabled FROM Currency ORDER BY currency, created_at DESC")
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

pub async fn lend(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all lend

    // time start
    let start = Instant::now();

    let all_lend = sqlx::query_as::<_, Lend>("SELECT DISTINCT ON (currency) created_at, currency, purchase_enable, redeem_enable, increment, min_purchase_size, max_purchase_size, interest_increment, min_interest_rate, market_interest_rate, max_interest_rate, auto_purchase_enable FROM Lend ORDER BY currency, created_at DESC")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let lend_with_index: Vec<(usize, Lend)> = all_lend
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = LendTemplate {
        lends: lend_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn borrow(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all borrow

    // time start
    let start = Instant::now();

    let all_borrow = sqlx::query_as::<_, Borrow>(
        "SELECT DISTINCT ON (currency) created_at, currency, hourly_borrow_rate, annualized_borrow_rate  FROM Borrow ORDER BY currency, created_at DESC",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let borrow_with_index: Vec<(usize, Borrow)> = all_borrow
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = BorrowTemplate {
        borrows: borrow_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn currency(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // one current currency

    // time start
    let start = Instant::now();

    let currency_name = path.into_inner();

    let currencies_with_one_currency_name = sqlx::query_as::<_, Currency>("SELECT created_at, currency, name, full_name, precision, confirms, contract_address, is_margin_enabled, is_debit_enabled FROM Currency WHERE currency = $1  ORDER BY created_at DESC").bind(&currency_name)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let currencies_with_index: Vec<(usize, Currency)> = currencies_with_one_currency_name
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = CurrencyTemplate {
        current_currency: currencies_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn ticker(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // one current ticker

    // time start
    let start = Instant::now();
    let symbol_name = path.into_inner();

    let tickers_with_one_symbol_name = sqlx::query_as::<_, Ticker>("SELECT created_at, symbol, symbol_name, buy, best_bid_size, sell, best_ask_size, change_rate, change_price, high, low, vol, vol_value, last, average_price, taker_fee_rate, maker_fee_rate, taker_coefficient, maker_coefficient FROM Ticker WHERE symbol_name = $1  ORDER BY created_at DESC").bind(&symbol_name)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let tickers_with_index: Vec<(usize, Ticker)> = tickers_with_one_symbol_name
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let chart_labels: Vec<String> = tickers_with_one_symbol_name
        .clone()
        .iter()
        .map(|t| t.created_at.format("%H:%M:%S").to_string())
        .collect();

    let chart_series: Vec<f64> = tickers_with_one_symbol_name
        .clone()
        .iter()
        .filter_map(|t| t.sell.as_ref().and_then(|s| s.parse::<f64>().ok()))
        .collect();

    let template: TickerTemplate = TickerTemplate {
        tickers: tickers_with_index,
        elapsed_ms: start.elapsed().as_millis(),
        chart_labels: chart_labels,
        chart_series: chart_series,
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn tickers(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all tickers

    // time start
    let start = Instant::now();

    let tickers = sqlx::query_as::<_, Ticker>("SELECT DISTINCT ON (symbol_name) created_at, symbol, symbol_name, buy, best_bid_size, sell, best_ask_size, change_rate, change_price, high, low, vol, vol_value, last, average_price, taker_fee_rate, maker_fee_rate, taker_coefficient, maker_coefficient FROM Ticker ORDER BY symbol_name, created_at DESC")
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

pub async fn serve_css() -> Result<HttpResponse, std::io::Error> {
    let content = std::fs::read_to_string("./static/style.css")?;

    Ok(HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(content))
}

pub async fn favicon() -> Result<HttpResponse, std::io::Error> {
    let bytes = std::fs::read("./static/favicon.png")?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(bytes))
}
