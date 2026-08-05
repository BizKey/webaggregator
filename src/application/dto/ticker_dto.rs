use crate::domain::entities::Ticker;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct TickerDto {
    pub exchange: String,
    pub symbol: String,
    pub symbol_name: String,
    pub taker_fee_rate: Option<f64>,
    pub maker_fee_rate: Option<f64>,
    pub taker_coefficient: Option<f64>,
    pub maker_coefficient: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

impl From<Ticker> for TickerDto {
    fn from(ticker: Ticker) -> Self {
        Self {
            exchange: ticker.exchange.as_str().to_string(),
            symbol: ticker.symbol.as_str().to_string(),
            symbol_name: ticker.symbol_name,
            taker_fee_rate: ticker.taker_fee_rate.map(|p| p.value()),
            maker_fee_rate: ticker.maker_fee_rate.map(|p| p.value()),
            taker_coefficient: ticker.taker_coefficient,
            maker_coefficient: ticker.maker_coefficient,
            updated_at: ticker.updated_at,
        }
    }
}
