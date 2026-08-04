use crate::domain::entities::Ticker;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct TickerModel {
    pub exchange: String,
    pub symbol: String,
    pub symbol_name: String,
    pub taker_fee_rate: Option<String>,
    pub maker_fee_rate: Option<String>,
    pub taker_coefficient: Option<f64>,
    pub maker_coefficient: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

impl From<TickerModel> for Ticker {
    fn from(model: TickerModel) -> Self {
        Self {
            exchange: model.exchange,
            symbol: model.symbol,
            symbol_name: model.symbol_name,
            taker_fee_rate: model.taker_fee_rate,
            maker_fee_rate: model.maker_fee_rate,
            taker_coefficient: model.taker_coefficient,
            maker_coefficient: model.maker_coefficient,
            updated_at: model.updated_at,
        }
    }
}
