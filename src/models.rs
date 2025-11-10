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
pub struct SymbolFee {
    pub fee_category: i16,
    pub taker_fee_coefficient: String,
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
pub struct CandleForStrategy {
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
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
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CandleForSma {
    pub symbol: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CandleClose {
    pub close: String,
}
// CandleWithProfit структура для хранения прибыльности по стратегии
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CandleWithProfit {
    pub symbol: String,
    pub profit: f64,
}
#[derive(Debug, sqlx::FromRow)]
pub struct CandleWithIncrement {
    pub symbol: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub price_increment: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Strategy {
    pub position: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
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

fn calculate_atr_for_candles(candles: &[CandleForStrategy], period: usize) -> Vec<Option<f64>> {
    if candles.len() < period {
        return vec![None; candles.len()];
    }

    let mut atr_values: Vec<Option<f64>> = vec![None; candles.len()];
    let mut true_ranges: Vec<f64> = Vec::new();

    // Рассчитываем True Range для всех свечей
    for i in 0..candles.len() {
        let high: f64 = candles[i].high.parse().unwrap_or(0.0);
        let low: f64 = candles[i].low.parse().unwrap_or(0.0);

        let tr = if i == 0 {
            high - low // для первой свечи TR = High - Low
        } else {
            let prev_close: f64 = candles[i - 1].close.parse().unwrap_or(0.0);
            let tr1 = high - low;
            let tr2 = (high - prev_close).abs();
            let tr3 = (low - prev_close).abs();
            tr1.max(tr2).max(tr3)
        };
        true_ranges.push(tr);
    }

    // Рассчитываем ATR
    for i in (period - 1)..candles.len() {
        if i == period - 1 {
            // Первое значение ATR - простая средняя первых period TR
            let sum: f64 = true_ranges[0..=i].iter().sum();
            atr_values[i] = Some(sum / period as f64);
        } else {
            // Последующие значения ATR - скользящая средняя
            let prev_atr = atr_values[i - 1].unwrap();
            let current_tr = true_ranges[i];
            atr_values[i] = Some((prev_atr * (period - 1) as f64 + current_tr) / period as f64);
        }
    }

    atr_values
}

pub fn round_to_decimal(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}
pub fn calc_strategy(
    candles: Vec<CandleForStrategy>,
    increment: &SymbolIncrement,
) -> Vec<Strategy> {
    let decimal_price_increment = get_decimal_places(&increment.price_increment);
    let mut strategies = Vec::new();
    let mut is_long = true;
    let position_size: f64 = 100.0;

    // Фиксированное соотношение риск:прибыль = 1:3
    let risk_reward_ratio = 3.0;
    let base_sl_atr = 2.0; // Базовый риск = 1 ATR

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

    let atr_values = calculate_atr_for_candles(&candles, 20);

    for (i, c) in candles.iter().enumerate() {
        let close_value = close_values[i];
        let current_atr = if i < atr_values.len() {
            atr_values[i].unwrap_or(0.0)
        } else {
            0.0
        };

        let (profit_price, loss_price, tp_per, sl_per) = if is_long {
            // Для лонга
            let sl_price = close_value - (current_atr * base_sl_atr);
            let risk_amount = close_value - sl_price;
            let tp_price = close_value + (risk_amount * risk_reward_ratio);

            let tp_percent = ((tp_price - close_value) / close_value) * 100.0;
            let sl_percent = ((close_value - sl_price) / close_value) * 100.0;
            (tp_price, sl_price, tp_percent, sl_percent)
        } else {
            // Для шорта
            let sl_price = close_value + (current_atr * base_sl_atr);
            let risk_amount = sl_price - close_value;
            let tp_price = close_value - (risk_amount * risk_reward_ratio);

            let tp_percent = ((close_value - tp_price) / close_value) * 100.0;
            let sl_percent = ((sl_price - close_value) / close_value) * 100.0;
            (tp_price, sl_price, tp_percent, sl_percent)
        };

        let result_trade = determine_trade_result(
            i,
            is_long,
            profit_price,
            loss_price,
            tp_per,
            sl_per,
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
            open: c.open.clone(),
            high: c.high.clone(),
            low: c.low.clone(),
            close: c.close.clone(),
            entry_price: c.close.clone(),
            profit_price: round_to_decimal(profit_price, decimal_price_increment),
            loss_price: round_to_decimal(loss_price, decimal_price_increment),
            position_size: position_size,
            result_trade: result_trade.trade_final,
            result_profit: round_to_decimal(result_trade.profit, 2),
            result_loss: round_to_decimal(result_trade.loss, 2),
            tp_per: round_to_decimal(tp_per, 2),
            sl_per: round_to_decimal(sl_per, 2),
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
    tp_per: f64, // теперь в процентах
    sl_per: f64, // теперь в процентах
    position_size: f64,
    high_values: &[f64],
    low_values: &[f64],
) -> TradeResult {
    for i in (entry_index + 1)..high_values.len() {
        let high = high_values[i];
        let low = low_values[i];

        if is_long {
            if low <= loss_point {
                return TradeResult {
                    trade_final: String::from("SL"),
                    profit: 0.0,
                    loss: position_size * (sl_per / 100.0), // используем sl_per вместо фиксированного sl
                };
            }
            if high >= profit_point {
                return TradeResult {
                    trade_final: String::from("TP"),
                    profit: position_size * (tp_per / 100.0), // используем tp_per вместо фиксированного tp
                    loss: 0.0,
                };
            }
        } else {
            if high >= loss_point {
                return TradeResult {
                    trade_final: String::from("SL"),
                    profit: 0.0,
                    loss: position_size * (sl_per / 100.0),
                };
            }
            if low <= profit_point {
                return TradeResult {
                    trade_final: String::from("TP"),
                    profit: position_size * (tp_per / 100.0),
                    loss: 0.0,
                };
            }
        }
    }

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

#[derive(Debug)]
pub struct SMAResult {
    pub period: usize,
    pub total_profit: f64,
    pub profit_percentage: f64,
    pub trades_count: usize,
    pub winning_trades: usize,
}
