use crate::domain::entities::Balance;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct BalanceDto {
    pub exchange: String,
    pub account_id: String,
    pub available: f64,
    pub available_change: f64,
    pub currency: String,
    pub hold_value: f64,
    pub hold_change: f64,
    pub relation_event: String,
    pub relation_event_id: String,
    pub event_time: String,
    pub total: f64,
    pub symbol: Option<String>,
    pub order_id: Option<String>,
    pub trade_id: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<Balance> for BalanceDto {
    fn from(balance: Balance) -> Self {
        Self {
            exchange: balance.exchange.as_str().to_string(),
            account_id: balance.account_id,
            available: balance.available.value(),
            available_change: balance.available_change.value(),
            currency: balance.currency,
            hold_value: balance.hold_value.value(),
            hold_change: balance.hold_change.value(),
            relation_event: balance.relation_event,
            relation_event_id: balance.relation_event_id,
            event_time: balance.event_time,
            total: balance.total.value(),
            symbol: balance.symbol.map(|s| s.as_str().to_string()),
            order_id: balance.order_id,
            trade_id: balance.trade_id,
            updated_at: balance.updated_at,
        }
    }
}
