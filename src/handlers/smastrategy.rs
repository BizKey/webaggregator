use crate::models::{CandleClose, CandleForSma, SMAResult};
use crate::templates::{CandlesCloseTemplate, CandlesSmaTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

fn simulate_sma_strategy(prices: &[f64], sma_period: usize) -> SMAResult {
    let sma_values = calculate_sma(prices, sma_period);
    let mut total_profit = 0.0;
    let mut trades_count = 0;
    let mut winning_trades = 0;
    let mut position: Option<f64> = None;

    for i in 0..prices.len() {
        if sma_values[i].is_none() {
            continue;
        }

        let current_price = prices[i];
        let current_sma_val = sma_values[i].unwrap();

        if current_price > current_sma_val {
            if position.is_none() {
                position = Some(current_price);
            }
        } else if current_price < current_sma_val {
            if let Some(buy_price) = position {
                let profit = 100.0 * (current_price / buy_price - 1.0);
                total_profit += profit;
                trades_count += 1;

                if profit > 0.0 {
                    winning_trades += 1;
                }

                position = None;
            }
        }
    }

    let profit_percentage = if trades_count > 0 {
        total_profit / (trades_count as f64 * 100.0) * 100.0
    } else {
        0.0
    };

    SMAResult {
        period: sma_period,
        total_profit,
        profit_percentage,
        trades_count,
        winning_trades,
    }
}

fn calculate_sma(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    if prices.len() < period {
        return vec![None; prices.len()];
    }

    let mut sma = vec![None; period - 1];
    let mut sum: f64 = prices[..period].iter().sum();

    for i in period..prices.len() {
        sma.push(Some(sum / period as f64));
        sum += prices[i] - prices[i - period];
    }

    if prices.len() >= period {
        sma.push(Some(sum / period as f64));
    }

    sma
}

fn find_best_sma_strategy(prices: &[f64]) -> Option<SMAResult> {
    if prices.len() < 24 {
        eprintln!("Недостаточно данных для анализа SMA");
        return None;
    }

    let mut best_result = None;
    let mut best_profit = std::f64::NEG_INFINITY;

    for period in 2..=200 {
        let result = simulate_sma_strategy(prices, period);

        if result.total_profit > best_profit {
            best_profit = result.total_profit;
            best_result = Some(result);
        }
    }

    best_result
}

pub async fn smastrategy_by_symbol(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    // sma strategy by symbol
    let symbol_name = path.into_inner();
    // time start
    let start = Instant::now();

    let candles = sqlx::query_as::<_, CandleClose>(
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

    let prices: Result<Vec<f64>, _> = candles
        .into_iter()
        .map(|c| c.close.parse::<f64>())
        .collect();

    let prices = prices.map_err(|e| {
        eprintln!("Error parsing prices: {}", e);
        actix_web::error::ErrorInternalServerError("Error parsing price data")
    })?;

    if prices.len() < 24 {
        return Ok(HttpResponse::BadRequest().body(format!(
            "Недостаточно данных для символа {}. Получено {} свечей, требуется минимум 24.",
            symbol_name,
            prices.len()
        )));
    }

    let best_sma = find_best_sma_strategy(&prices);

    let response = match best_sma {
        Some(result) => {
            let win_rate = if result.trades_count > 0 {
                (result.winning_trades as f64 / result.trades_count as f64) * 100.0
            } else {
                0.0
            };

            format!(
                "SMA Strategy Analysis for {}:\n\
                 Best SMA Period: {}\n\
                 Total Profit: ${:.2}\n\
                 Average Profit per Trade: {:.1}%\n\
                 Total Trades: {}\n\
                 Winning Trades: {} ({:.1}% win rate)\n\
                 Analysis Time: {}ms\n\
                 Data Points: {}",
                symbol_name,
                result.period,
                result.total_profit,
                result.profit_percentage,
                result.trades_count,
                result.winning_trades,
                win_rate,
                start.elapsed().as_millis(),
                prices.len()
            )
        }
        None => {
            format!(
                "Не удалось найти прибыльную SMA стратегию для {}\n\
                 Analysis Time: {}ms",
                symbol_name,
                start.elapsed().as_millis()
            )
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(response))
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
