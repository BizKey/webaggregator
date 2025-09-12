use actix_web::{HttpResponse, Result, web};
use askama::Template;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;

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
}
#[derive(Template)]
#[template(path = "ticker.html")]
struct TickerTemplate {
    tickers: Vec<(usize, Ticker)>,
}
#[derive(Template)]
#[template(path = "symbols.html")]
struct SymbolsTemplate {
    symbols: Vec<Symbol>,
}

#[derive(Template)]
#[template(path = "currencies.html")]
struct CurrencyTemplate {
    currencies: Vec<Currency>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
    time: String,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Welcome to rr")
}

pub async fn hello() -> HttpResponse {
    let template = IndexTemplate {
        name: "User".to_string(),
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
    };

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
    let currencies = sqlx::query_as::<_, Currency>("SELECT created_at, currency, name, full_name, precision, confirms, contract_address, is_margin_enabled, is_debit_enabled FROM Currency")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let template = CurrencyTemplate { currencies };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn ticker(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // one current ticker
    let symbol_name = path.into_inner();

    let one_ticker = sqlx::query_as::<_, Ticker>("SELECT created_at, symbol, symbol_name, buy, best_bid_size, sell, best_ask_size, change_rate, change_price, high, low, vol, vol_value, last, average_price, taker_fee_rate, maker_fee_rate, taker_coefficient, maker_coefficient FROM Ticker WHERE symbol_name = $1").bind(&symbol_name)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let tickers_with_index: Vec<(usize, Ticker)> = one_ticker
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template = TickerTemplate {
        tickers: tickers_with_index,
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
    let tickers = sqlx::query_as::<_, Ticker>("SELECT created_at, symbol, symbol_name, buy, best_bid_size, sell, best_ask_size, change_rate, change_price, high, low, vol, vol_value, last, average_price, taker_fee_rate, maker_fee_rate, taker_coefficient, maker_coefficient FROM Ticker")
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
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn hellodirect(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let name = path.into_inner();

    let _ = sqlx::query("INSERT INTO Ticker (symbol, symbol_name) VALUES ($1,$2)")
        .bind(&name)
        .bind(&name)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to create ticket")
        })?;

    let template = IndexTemplate {
        name: name,
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
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
