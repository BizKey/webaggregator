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
            base_min_size: symbol.base_min_size,
            quote_min_size: symbol.quote_min_size,
            base_max_size: symbol.base_max_size,
            quote_max_size: symbol.quote_max_size,
            base_increment: symbol.base_increment,
            quote_increment: symbol.quote_increment,
            price_increment: symbol.price_increment,
            price_limit_rate: symbol.price_limit_rate,
            min_funds: symbol.min_funds,
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
