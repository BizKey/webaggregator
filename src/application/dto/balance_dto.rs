use crate::domain::entities::Balance;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct BalanceDto {
    pub exchange: String,
    pub account_id: String,
    pub available: String,
    pub available_change: String,
    pub currency: String,
    pub hold_value: String,
    pub hold_change: String,
    pub relation_event: String,
    pub relation_event_id: String,
    pub event_time: String,
    pub total: String,
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
            available: balance.available,
            available_change: balance.available_change,
            currency: balance.currency,
            hold_value: balance.hold_value,
            hold_change: balance.hold_change,
            relation_event: balance.relation_event,
            relation_event_id: balance.relation_event_id,
            event_time: balance.event_time,
            total: balance.total,
            symbol: balance.symbol.map(|s| s.as_str().to_string()),
            order_id: balance.order_id,
            trade_id: balance.trade_id,
            updated_at: balance.updated_at,
        }
    }
}
