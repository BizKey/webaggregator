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
pub struct SymbolIncrement {
    pub price_increment: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Total {
    pub total: f64,
    pub total_loss: f64,
    pub total_profit: f64,
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

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
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
// CandleWithProfit структура для хранения прибыльности по стратегии
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CandleWithProfit {
    pub exchange: String,
    pub symbol: String,
    pub profit: f64,
}
#[derive(Debug, sqlx::FromRow)]
pub struct CandleWithIncrement {
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
    pub price_increment: String,
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
    pub entry_price: String,
    pub profit_price: f64,
    pub loss_price: f64,
    pub position_size: f64,
    pub result_trade: String,
    pub result_profit: f64,
    pub result_loss: f64,
    pub tp_per: f64,
    pub sl_per: f64,
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
fn get_decimal_places(precision_str: &str) -> u32 {
    if let Some(dot_pos) = precision_str.find('.') {
        (precision_str.len() - dot_pos - 1) as u32
    } else {
        0
    }
}

fn round_to_decimal(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}
pub fn calc_strategy(candles: Vec<Candle>, increment: &SymbolIncrement) -> Vec<Strategy> {
    let decimal_price_increment = get_decimal_places(&increment.price_increment);
    let mut strategies = Vec::new();
    let mut is_long = true;
    let position_size: f64 = 100.0;
    let tp: f64 = 6.0;
    let sl: f64 = 2.0;

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
        let (profit_price, loss_price) = if is_long {
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

        let result_trade = determine_trade_result(
            i,
            is_long,
            profit_price,
            loss_price,
            tp,
            sl,
            position_size,
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
            entry_price: c.close.clone(),
            profit_price: round_to_decimal(profit_price, decimal_price_increment),
            loss_price: round_to_decimal(loss_price, decimal_price_increment),
            position_size: position_size,
            result_trade: result_trade.trade_final,
            result_profit: result_trade.profit,
            result_loss: result_trade.loss,
            tp_per: tp,
            sl_per: sl,
        });

        is_long = !is_long;
    }

    strategies
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct TradeResult {
    trade_final: String,
    profit: f64,
    loss: f64,
}

fn determine_trade_result(
    entry_index: usize,
    is_long: bool,
    profit_point: f64,
    loss_point: f64,
    tp: f64,
    sl: f64,
    position_size: f64,
    high_values: &[f64],
    low_values: &[f64],
) -> TradeResult {
    // Ищем в последующих свечах, что сработало первое - TP или SL
    for i in (entry_index + 1)..high_values.len() {
        let high = high_values[i];
        let low = low_values[i];

        if is_long {
            // Для лонга: TP - когда high достиг profit_point, SL - когда low достиг loss_point
            if low <= loss_point {
                return TradeResult {
                    trade_final: String::from("SL"),
                    profit: 0.0,
                    loss: position_size * (sl / 100.0),
                };
            }
            if high >= profit_point {
                return TradeResult {
                    trade_final: String::from("TP"),
                    profit: position_size * (tp / 100.0),
                    loss: 0.0,
                };
            }
        } else {
            // Для шорта: TP - когда low достиг profit_point, SL - когда high достиг loss_point
            if high >= loss_point {
                return TradeResult {
                    trade_final: String::from("SL"),
                    profit: 0.0,
                    loss: position_size * (sl / 100.0),
                };
            }
            if low <= profit_point {
                return TradeResult {
                    trade_final: String::from("TP"),
                    profit: position_size * (tp / 100.0),
                    loss: 0.0,
                };
            }
        }
    }

    // Если не сработал ни TP ни SL до конца данных
    TradeResult {
        trade_final: String::from("Open"),
        profit: 0.0,
        loss: 0.0,
    }
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
