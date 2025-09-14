use actix_web::{HttpResponse, Result, web};
use askama::Template;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use std::time::Instant;

pub trait DateTimeFormat {
    fn format_date(&self) -> String;
}

impl DateTimeFormat for DateTime<Utc> {
    fn format_date(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticker {
    pub created_at: DateTime<Utc>,
    pub symbol: String,
    pub symbol_name: String,
    pub buy: Option<String>,
    pub best_bid_size: Option<String>,
    pub sell: Option<String>,
    pub best_ask_size: Option<String>,
    pub change_rate: Option<String>,
    pub change_price: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    pub vol: Option<String>,
    pub vol_value: Option<String>,
    pub last: Option<String>,
    pub average_price: Option<String>,
    pub taker_fee_rate: Option<String>,
    pub maker_fee_rate: Option<String>,
    pub taker_coefficient: Option<String>,
    pub maker_coefficient: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Symbol {
    pub created_at: DateTime<Utc>,
    pub symbol: String,
    pub name: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub fee_currency: String,
    pub market: String,
    pub base_min_size: String,
    pub quote_min_size: String,
    pub base_max_size: String,
    pub quote_max_size: String,
    pub base_increment: String,
    pub quote_increment: String,
    pub price_increment: String,
    pub price_limit_rate: String,
    pub min_funds: String,
    pub is_margin_enabled: bool,
    pub enable_trading: bool,
    pub fee_category: i64,
    pub maker_fee_coefficient: String,
    pub taker_fee_coefficient: String,
    pub st: bool,
    pub callauction_is_enabled: bool,
    pub callauction_price_floor: Option<String>,
    pub callauction_price_ceiling: Option<String>,
    pub callauction_first_stage_start_time: Option<i64>,
    pub callauction_second_stage_start_time: Option<i64>,
    pub callauction_third_stage_start_time: Option<i64>,
    pub trading_start_time: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Currency {
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub name: String,
    pub full_name: String,
    pub precision: i16,
    pub confirms: Option<i16>,
    pub contract_address: Option<String>,
    pub is_margin_enabled: bool,
    pub is_debit_enabled: bool,
}

#[derive(Template)]
#[template(path = "tickers.html")]
struct TickersTemplate {
    tickers: Vec<(usize, Ticker)>,
    elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "ticker.html")]
struct TickerTemplate {
    tickers: Vec<(usize, Ticker)>,
    elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "symbols.html")]
struct SymbolsTemplate {
    symbols: Vec<Symbol>,
}

#[derive(Template)]
#[template(path = "currencies.html")]
struct CurrenciesTemplate {
    currencies: Vec<(usize, Currency)>,
    elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "currency.html")]
struct CurrencyTemplate {
    current_currency: Vec<(usize, Currency)>,
    elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

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
    let symbols = sqlx::query_as::<_, Symbol>("SELECT created_at FROM Symbol")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let template = SymbolsTemplate { symbols };
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

    let all_currencies = sqlx::query_as::<_, Currency>("SELECT created_at, currency, name, full_name, precision, confirms, contract_address, is_margin_enabled, is_debit_enabled FROM Currency")
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

    // time end
    let elapsed_ms = start.elapsed().as_millis();

    let template = CurrenciesTemplate {
        currencies: currencies_with_index,
        elapsed_ms: elapsed_ms,
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

    // time end
    let elapsed_ms = start.elapsed().as_millis();

    let template = CurrencyTemplate {
        current_currency: currencies_with_index,
        elapsed_ms: elapsed_ms,
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
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    // time end
    let elapsed_ms = start.elapsed().as_millis();

    let template = TickerTemplate {
        tickers: tickers_with_index,
        elapsed_ms: elapsed_ms,
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

    // time end
    let elapsed_ms = start.elapsed().as_millis();

    let template = TickersTemplate {
        tickers: tickers_with_index,
        elapsed_ms: elapsed_ms,
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
