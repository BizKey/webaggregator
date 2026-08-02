use crate::domain::entities::Currency;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CurrencyDto {
    pub exchange: String,
    pub currency: String,
    pub currency_name: String,
    pub full_name: String,
    pub precision: i16,
    pub is_margin_enabled: bool,
    pub is_debit_enabled: bool,
    pub updated_at: DateTime<Utc>,
}

impl From<Currency> for CurrencyDto {
    fn from(currency: Currency) -> Self {
        Self {
            exchange: currency.exchange.as_str().to_string(),
            currency: currency.currency,
            currency_name: currency.currency_name,
            full_name: currency.full_name,
            precision: currency.precision,
            is_margin_enabled: currency.is_margin_enabled,
            is_debit_enabled: currency.is_debit_enabled,
            updated_at: currency.updated_at,
        }
    }
}
