use crate::domain::entities::Symbol;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SymbolDto {
    pub exchange: String,
    pub symbol: String,
    pub symbol_name: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub fee_currency: String,
    pub market: String,
    pub base_min_size: f64,
    pub quote_min_size: f64,
    pub base_max_size: f64,
    pub quote_max_size: f64,
    pub base_increment: f64,
    pub quote_increment: f64,
    pub price_increment: f64,
    pub price_limit_rate: String,
    pub min_funds: Option<f64>,
    pub is_margin_enabled: bool,
    pub enable_trading: bool,
    pub fee_category: i16,
    pub maker_fee_coefficient: f64,
    pub taker_fee_coefficient: f64,
    pub st: bool,
    pub updated_at: DateTime<Utc>,
}

impl From<Symbol> for SymbolDto {
    fn from(symbol: Symbol) -> Self {
        Self {
            exchange: symbol.exchange.as_str().to_string(),
            symbol: symbol.symbol.as_str().to_string(),
            symbol_name: symbol.symbol_name,
            base_currency: symbol.base_currency,
            quote_currency: symbol.quote_currency,
            fee_currency: symbol.fee_currency,
            market: symbol.market,
            base_min_size: symbol.base_min_size.value(),
            quote_min_size: symbol.quote_min_size.value(),
            base_max_size: symbol.base_max_size.value(),
            quote_max_size: symbol.quote_max_size.value(),
            base_increment: symbol.base_increment.value(),
            quote_increment: symbol.quote_increment.value(),
            price_increment: symbol.price_increment.value(),
            price_limit_rate: symbol.price_limit_rate,
            min_funds: symbol.min_funds.map(|m| m.value()),
            is_margin_enabled: symbol.is_margin_enabled,
            enable_trading: symbol.enable_trading,
            fee_category: symbol.fee_category,
            maker_fee_coefficient: symbol.maker_fee_coefficient,
            taker_fee_coefficient: symbol.taker_fee_coefficient,
            st: symbol.st,
            updated_at: symbol.updated_at,
        }
    }
}
