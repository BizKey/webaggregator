use crate::domain::value_objects::Exchange;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub exchange: Exchange,
    pub currency: String,
    pub currency_name: String,
    pub full_name: String,
    pub precision: i16,
    pub is_margin_enabled: bool,
    pub is_debit_enabled: bool,
    pub updated_at: DateTime<Utc>,
}

impl Currency {
    pub fn new(
        exchange: Exchange,
        currency: String,
        currency_name: String,
        full_name: String,
        precision: i16,
        is_margin_enabled: bool,
        is_debit_enabled: bool,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            currency,
            currency_name,
            full_name,
            precision,
            is_margin_enabled,
            is_debit_enabled,
            updated_at,
        }
    }
}
