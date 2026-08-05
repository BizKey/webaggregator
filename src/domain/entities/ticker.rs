use crate::domain::value_objects::{Exchange, SymbolName};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub exchange: Exchange,
    pub symbol: SymbolName,
    pub symbol_name: String,
    pub taker_fee_rate: Option<String>,
    pub maker_fee_rate: Option<String>,
    pub taker_coefficient: Option<f64>,
    pub maker_coefficient: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

impl Ticker {
    pub fn new(
        exchange: Exchange,
        symbol: SymbolName,
        symbol_name: String,
        taker_fee_rate: Option<String>,
        maker_fee_rate: Option<String>,
        taker_coefficient: Option<f64>,
        maker_coefficient: Option<f64>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            symbol,
            symbol_name,
            taker_fee_rate,
            maker_fee_rate,
            taker_coefficient,
            maker_coefficient,
            updated_at,
        }
    }
}
