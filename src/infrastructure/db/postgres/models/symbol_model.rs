use crate::domain::entities::Symbol;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct SymbolModel {
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

impl From<SymbolModel> for Symbol {
    fn from(model: SymbolModel) -> Self {
        Self {
            exchange: model.exchange,
            symbol: model.symbol,
            symbol_name: model.symbol_name,
            base_currency: model.base_currency,
            quote_currency: model.quote_currency,
            fee_currency: model.fee_currency,
            market: model.market,
            base_min_size: model.base_min_size,
            quote_min_size: model.quote_min_size,
            base_max_size: model.base_max_size,
            quote_max_size: model.quote_max_size,
            base_increment: model.base_increment,
            quote_increment: model.quote_increment,
            price_increment: model.price_increment,
            price_limit_rate: model.price_limit_rate,
            min_funds: model.min_funds,
            is_margin_enabled: model.is_margin_enabled,
            enable_trading: model.enable_trading,
            fee_category: model.fee_category,
            maker_fee_coefficient: model.maker_fee_coefficient,
            taker_fee_coefficient: model.taker_fee_coefficient,
            st: model.st,
            updated_at: model.updated_at,
        }
    }
}
