use actix_web::{HttpResponse, Result, web};
use askama::Template;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
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

#[derive(Template)]
#[template(path = "tickers.html")]
struct TickersTemplate {
    tickets: Vec<Ticket>,
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

pub async fn tickers(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let tickets = sqlx::query_as::<_, Ticket>("SELECT symbol, symbol_name, buy, best_bid_size, sell, best_ask_size, change_rate, change_price, high, low, vol, vol_value, last, average_price, taker_fee_rate, maker_fee_rate, taker_coefficient, maker_coefficient, created_at FROM Ticker")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let template = TickersTemplate { tickets };
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
