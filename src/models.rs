use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Ticker {
    pub exchange: String,
    pub symbol: String,
    pub symbol_name: String,
    pub taker_fee_rate: Option<String>,
    pub maker_fee_rate: Option<String>,
    pub taker_coefficient: Option<String>,
    pub maker_coefficient: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Symbol {
    pub exchange: String,
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
    pub min_funds: Option<String>,
    pub is_margin_enabled: bool,
    pub enable_trading: bool,
    pub fee_category: i16,
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
    pub exchange: String,
    pub currency: String,
    pub name: String,
    pub full_name: String,
    pub precision: i16,
    pub confirms: Option<i16>,
    pub contract_address: Option<String>,
    pub is_margin_enabled: bool,
    pub is_debit_enabled: bool,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Lend {
    pub exchange: String,
    pub currency: String,
    pub purchase_enable: bool,
    pub redeem_enable: bool,
    pub increment: String,
    pub min_purchase_size: String,
    pub max_purchase_size: String,
    pub interest_increment: String,
    pub min_interest_rate: String,
    pub market_interest_rate: String,
    pub max_interest_rate: String,
    pub auto_purchase_enable: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Borrow {
    pub exchange: String,
    pub currency: String,
    pub hourly_borrow_rate: String,
    pub annualized_borrow_rate: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Candle {
    pub exchange: String,
    pub symbol: String,
    pub interval: String,
    pub timestamp: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub quote_volume: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Strategy {
    pub position: String,
    pub exchange: String,
    pub symbol: String,
    pub interval: String,
    pub timestamp: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub quote_volume: String,
    pub entry_point: String,
    pub profit_point: f64,
    pub loss_point: f64,
    pub position_size: f64,
    pub result_trade: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleWithAtr {
    pub exchange: String,
    pub symbol: String,
    pub interval: String,
    pub timestamp: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub quote_volume: String,
    pub atr: Option<f64>,
    pub atr_percent: Option<f64>,
}

fn calculate_true_range(current: &Candle, previous: Option<&Candle>) -> f64 {
    let high: f64 = current.high.parse().unwrap_or(0.0);
    let low: f64 = current.low.parse().unwrap_or(0.0);

    if let Some(prev) = previous {
        let prev_close: f64 = prev.close.parse().unwrap_or(0.0);

        let tr1 = high - low;
        let tr2 = (high - prev_close).abs();
        let tr3 = (low - prev_close).abs();

        tr1.max(tr2).max(tr3)
    } else {
        high - low
    }
}

pub fn calc_strategy(candles: Vec<Candle>) -> Vec<Strategy> {
    let mut strategies = Vec::new();
    let mut is_long = true;
    let position_size: f64 = 100.0;
    let tp: f64 = 5.0;
    let sl: f64 = 1.5;

    // Преобразуем все close значения заранее для эффективности
    let close_values: Vec<f64> = candles
        .iter()
        .map(|c| c.close.parse().unwrap_or(0.0))
        .collect();

    let high_values: Vec<f64> = candles
        .iter()
        .map(|c| c.high.parse().unwrap_or(0.0))
        .collect();

    let low_values: Vec<f64> = candles
        .iter()
        .map(|c| c.low.parse().unwrap_or(0.0))
        .collect();

    for (i, c) in candles.iter().enumerate() {
        let close_value = close_values[i];
        let (profit_point, loss_point) = if is_long {
            (
                close_value * (1.0 + tp / 100.0),
                close_value * (1.0 - sl / 100.0),
            )
        } else {
            (
                close_value * (1.0 - tp / 100.0),
                close_value * (1.0 + sl / 100.0),
            )
        };

        // Определяем результат сделки
        let result_trade = determine_trade_result(
            i,
            is_long,
            profit_point,
            loss_point,
            &high_values,
            &low_values,
        );

        strategies.push(Strategy {
            position: if is_long {
                String::from("Long")
            } else {
                String::from("Short")
            },
            exchange: c.exchange.clone(),
            symbol: c.symbol.clone(),
            interval: c.interval.clone(),
            timestamp: c.timestamp.clone(),
            open: c.open.clone(),
            high: c.high.clone(),
            low: c.low.clone(),
            close: c.close.clone(),
            volume: c.volume.clone(),
            quote_volume: c.quote_volume.clone(),
            entry_point: c.close.clone(),
            profit_point: profit_point,
            loss_point: loss_point,
            position_size: position_size,
            result_trade, // Добавляем результат сделки
        });

        is_long = !is_long;
    }

    strategies
}

fn determine_trade_result(
    entry_index: usize,
    is_long: bool,
    profit_point: f64,
    loss_point: f64,
    high_values: &[f64],
    low_values: &[f64],
) -> String {
    // Ищем в последующих свечах, что сработало первое - TP или SL
    for i in (entry_index + 1)..high_values.len() {
        let high = high_values[i];
        let low = low_values[i];

        if is_long {
            // Для лонга: TP - когда high достиг profit_point, SL - когда low достиг loss_point
            if high >= profit_point {
                return String::from("TP");
            }
            if low <= loss_point {
                return String::from("SL");
            }
        } else {
            // Для шорта: TP - когда low достиг profit_point, SL - когда high достиг loss_point
            if low <= profit_point {
                return String::from("TP");
            }
            if high >= loss_point {
                return String::from("SL");
            }
        }
    }

    // Если не сработал ни TP ни SL до конца данных
    String::from("Open")
}

pub fn calculate_atr(candles: &[Candle], period: usize) -> Vec<CandleWithAtr> {
    if candles.len() < period {
        return candles
            .iter()
            .map(|c| CandleWithAtr {
                exchange: c.exchange.clone(),
                symbol: c.symbol.clone(),
                interval: c.interval.clone(),
                timestamp: c.timestamp.clone(),
                open: c.open.clone(),
                high: c.high.clone(),
                low: c.low.clone(),
                close: c.close.clone(),
                volume: c.volume.clone(),
                quote_volume: c.quote_volume.clone(),
                atr: None,
                atr_percent: None,
            })
            .collect();
    }

    let mut result: Vec<CandleWithAtr> = Vec::new();
    let mut true_ranges: Vec<f64> = Vec::new();

    for i in 0..candles.len() {
        let previous = if i > 0 { Some(&candles[i - 1]) } else { None };
        let tr = calculate_true_range(&candles[i], previous);
        true_ranges.push(tr);
    }

    for i in 0..candles.len() {
        let atr = if i < period - 1 {
            None
        } else if i == period - 1 {
            let sum: f64 = true_ranges[0..=i].iter().sum();
            Some(sum / period as f64)
        } else {
            let prev_atr = result[i - 1].atr.unwrap();
            let current_tr = true_ranges[i];
            Some((prev_atr * (period - 1) as f64 + current_tr) / period as f64)
        };

        result.push(CandleWithAtr {
            exchange: candles[i].exchange.clone(),
            symbol: candles[i].symbol.clone(),
            interval: candles[i].interval.clone(),
            timestamp: candles[i].timestamp.clone(),
            open: candles[i].open.clone(),
            high: candles[i].high.clone(),
            low: candles[i].low.clone(),
            close: candles[i].close.clone(),
            volume: candles[i].volume.clone(),
            quote_volume: candles[i].quote_volume.clone(),
            atr: atr,
            atr_percent: None,
        });
    }

    result
}
