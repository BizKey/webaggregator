use crate::models::{
    Borrow, Candle, CandleProfit, CandleWithAtr, Currency, Lend, Strategy, Symbol, SymbolIncrement,
    Ticker, Total, calc_strategy, calculate_atr,
};
use crate::templates::{
    BorrowTemplate, BorrowsTemplate, CandleTemplate, CandlesTemplate, CurrenciesTemplate,
    IndexTemplate, LendTemplate, LendsTemplate, OneStrategyTemplate, StrategyTemplate,
    SymbolsTemplate, TickersTemplate,
};
use actix_web::{HttpResponse, Result, web};
use askama::Template;
use std::collections::HashMap;

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

    let symbols = sqlx::query_as::<_, Symbol>(
        "SELECT 
                exchange, symbol, name, base_currency, quote_currency, fee_currency, 
                market, base_min_size, quote_min_size, base_max_size, quote_max_size, 
                base_increment, quote_increment, price_increment, price_limit_rate, 
                min_funds, is_margin_enabled, enable_trading, fee_category, 
                maker_fee_coefficient, taker_fee_coefficient, st, callauction_is_enabled, 
                callauction_price_floor, callauction_price_ceiling, 
                callauction_first_stage_start_time, callauction_second_stage_start_time, 
                callauction_third_stage_start_time, trading_start_time 
            FROM symbol",
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

pub async fn currencies(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all currency

    // time start
    let start = Instant::now();

    let all_currencies = sqlx::query_as::<_, Currency>(
        "SELECT 
                exchange, currency, name, full_name, precision, confirms, 
                contract_address, is_margin_enabled, is_debit_enabled 
            FROM currency",
    )
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

pub async fn lends(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all lend

    // time start
    let start = Instant::now();

    let all_lend = sqlx::query_as::<_, Lend>(
        "SELECT DISTINCT ON (currency) 
                exchange, currency, purchase_enable, redeem_enable, increment, 
                min_purchase_size, max_purchase_size, interest_increment, 
                min_interest_rate, market_interest_rate, max_interest_rate, 
                auto_purchase_enable 
            FROM lend",
    )
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

    let template = LendsTemplate {
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
pub async fn lend(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all lend

    // time start
    let start = Instant::now();
    let currency_name = path.into_inner();

    let all_lend = sqlx::query_as::<_, Lend>(
        "SELECT 
                exchange, currency, purchase_enable, redeem_enable, increment, 
                min_purchase_size, max_purchase_size, interest_increment, 
                min_interest_rate, market_interest_rate, max_interest_rate, 
                auto_purchase_enable 
            FROM lend 
            WHERE currency = $1",
    )
    .bind(&currency_name)
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

pub async fn strategy(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let start = Instant::now();

    let symbols =
        sqlx::query_scalar::<_, String>("SELECT DISTINCT symbol FROM candle ORDER BY symbol")
            .fetch_all(pool.get_ref())
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?;

    let symbol_increments_futures: Vec<_> = symbols
        .iter()
        .map(|symbol| {
            sqlx::query_as::<_, SymbolIncrement>(
                "SELECT price_increment FROM symbol WHERE symbol = $1",
            )
            .bind(symbol)
            .fetch_one(pool.get_ref())
        })
        .collect();

    let symbol_increments = futures::future::try_join_all(symbol_increments_futures)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let increment_map: HashMap<_, _> = symbols
        .iter()
        .zip(symbol_increments.iter())
        .map(|(symbol, increment)| (symbol.as_str(), increment))
        .collect();

    let candles_futures: Vec<_> = symbols.iter().map(|symbol| {
        sqlx::query_as::<_, Candle>(
            "SELECT exchange, symbol, interval, timestamp, open, high, low, close, volume, quote_volume
             FROM candle 
             WHERE symbol = $1
             ORDER BY timestamp::BIGINT ASC"
        )
        .bind(symbol)
        .fetch_all(pool.get_ref())
    }).collect();

    let all_candles_results = futures::future::try_join_all(candles_futures)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let mut candle_with_profit = Vec::with_capacity(symbols.len());

    for (symbol, candles) in symbols.iter().zip(all_candles_results.iter()) {
        if let Some(increment) = increment_map.get(symbol.as_str()) {
            if let Some(latest_candle) = candles.last() {
                let processed_candles = calc_strategy(candles.clone(), increment);

                let total_profit: f64 = processed_candles.iter().map(|s| s.result_profit).sum();
                let total_loss: f64 = processed_candles.iter().map(|s| s.result_loss).sum();
                let net_result = total_profit - total_loss;

                candle_with_profit.push(CandleProfit {
                    exchange: latest_candle.exchange.clone(),
                    symbol: latest_candle.symbol.clone(),
                    interval: latest_candle.interval.clone(),
                    open: latest_candle.open.clone(),
                    timestamp: latest_candle.timestamp.clone(),
                    high: latest_candle.high.clone(),
                    low: latest_candle.low.clone(),
                    close: latest_candle.close.clone(),
                    volume: latest_candle.volume.clone(),
                    quote_volume: latest_candle.quote_volume.clone(),
                    profit: net_result,
                });
            }
        }
    }

    // Остальной код без изменений
    let candles_with_index: Vec<(usize, CandleProfit)> = candle_with_profit
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template = StrategyTemplate {
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

pub async fn borrows(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all borrow

    // time start
    let start = Instant::now();

    let all_borrow = sqlx::query_as::<_, Borrow>(
        "SELECT DISTINCT ON (currency) 
                exchange, currency, hourly_borrow_rate, annualized_borrow_rate 
            FROM borrow",
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

    let template = BorrowsTemplate {
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

pub async fn borrow(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all borrow

    // time start
    let start = Instant::now();
    let currency_name = path.into_inner();

    let all_borrow = sqlx::query_as::<_, Borrow>(
        "SELECT 
                exchange, currency, hourly_borrow_rate, annualized_borrow_rate 
            FROM borrow 
            WHERE currency = $1",
    )
    .bind(&currency_name)
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

pub async fn tickerstrategy(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    // time start
    let start = Instant::now();
    let symbol_name = path.into_inner();

    let candles = sqlx::query_as::<_, Candle>(
        "SELECT exchange, symbol, interval, timestamp, open, high, low, close, volume, quote_volume
            FROM candle 
            WHERE symbol = $1
            ORDER BY symbol, timestamp::BIGINT ASC",
    )
    .bind(&symbol_name)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let increment = sqlx::query_as::<_, SymbolIncrement>(
        "SELECT price_increment FROM symbol WHERE symbol = $1",
    )
    .bind(&symbol_name)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let processed_candles: Vec<Strategy> = calc_strategy(candles, &increment);

    let total_profit: f64 = processed_candles.iter().map(|s| s.result_profit).sum();
    let total_loss: f64 = processed_candles.iter().map(|s| s.result_loss).sum();
    let net_result: f64 = total_profit - total_loss;

    let template = OneStrategyTemplate {
        candles: processed_candles,
        total: Total {
            total: net_result,
            total_loss: total_loss,
            total_profit: total_profit,
        },
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

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

pub async fn tickers(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all tickers

    // time start
    let start = Instant::now();

    let tickers = sqlx::query_as::<_, Ticker>(
        "SELECT
                exchange, symbol, symbol_name, taker_fee_rate, maker_fee_rate, 
                taker_coefficient, maker_coefficient 
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
