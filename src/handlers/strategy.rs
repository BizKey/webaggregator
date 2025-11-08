use crate::models::{
    CandleForStrategy, CandleWithIncrement, CandleWithProfit, Strategy, SymbolIncrement, Total,
    calc_strategy, round_to_decimal,
};
use crate::templates::{OneStrategyTemplate, StrategyTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;
use std::collections::HashMap;

use sqlx::PgPool;
use std::time::Instant;

pub async fn strategy(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let start = Instant::now();

    let candle_data: Vec<CandleWithIncrement> = sqlx::query_as::<_, CandleWithIncrement>(
        r#"
        SELECT 
            c.symbol,  
            c.open, 
            c.high, 
            c.low, 
            c.close, 
            s.price_increment
        FROM candle c
        JOIN symbol s ON c.symbol = s.symbol
        ORDER BY c.symbol, c.timestamp::BIGINT ASC
        "#,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let mut candles_by_symbol: HashMap<String, (SymbolIncrement, Vec<CandleForStrategy>)> =
        HashMap::new();

    for data in candle_data {
        let entry = candles_by_symbol
            .entry(data.symbol.clone())
            .or_insert_with(|| {
                (
                    SymbolIncrement {
                        price_increment: data.price_increment.to_string(),
                    },
                    Vec::new(),
                )
            });

        entry.1.push(CandleForStrategy {
            open: data.open,
            high: data.high,
            low: data.low,
            close: data.close,
        });
    }

    let mut candle_with_profit: Vec<CandleWithProfit> = Vec::with_capacity(candles_by_symbol.len());

    for (symbol, (increment, candles)) in candles_by_symbol {
        candle_with_profit.push(CandleWithProfit {
            symbol: symbol,
            profit: round_to_decimal(
                calc_strategy(candles.clone(), &increment)
                    .iter()
                    .map(|s| s.result_profit - s.result_loss)
                    .sum(),
                2,
            ),
        });
    }

    let total_profit: f64 = candle_with_profit.iter().map(|s| s.profit).sum();
    candle_with_profit.sort_by(|a, b| {
        b.profit
            .partial_cmp(&a.profit)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let candles_with_index: Vec<(usize, CandleWithProfit)> = candle_with_profit
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template: StrategyTemplate = StrategyTemplate {
        candles: candles_with_index,
        total_profit: total_profit,
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

    let candles: Vec<CandleForStrategy> = sqlx::query_as::<_, CandleForStrategy>(
        "SELECT open, high, low, close
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
            total: round_to_decimal(net_result, 2),
            total_loss: round_to_decimal(total_loss, 2),
            total_profit: round_to_decimal(total_profit, 2),
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
