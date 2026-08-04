use crate::domain::entities::Currency;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CurrencyModel {
    pub exchange: String,
    pub currency: String,
    pub currency_name: String,
    pub full_name: String,
    pub precision: i16,
    pub is_margin_enabled: bool,
    pub is_debit_enabled: bool,
    pub updated_at: DateTime<Utc>,
}

impl From<CurrencyModel> for Currency {
    fn from(model: CurrencyModel) -> Self {
        Self {
            exchange: model.exchange,
            currency: model.currency,
            currency_name: model.currency_name,
            full_name: model.full_name,
            precision: model.precision,
            is_margin_enabled: model.is_margin_enabled,
            is_debit_enabled: model.is_debit_enabled,
            updated_at: model.updated_at,
        }
    }
}
