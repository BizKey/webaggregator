use crate::domain::value_objects::{Money};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub exchange: String,
    pub account_id: String,
    pub available: Money,
    pub available_change: Money,
    pub currency: String,
    pub hold_value: Money,
    pub hold_change: Money,
    pub relation_event: String,
    pub relation_event_id: String,
    pub event_time: String,
    pub total: Money,
    pub symbol: Option<String>,
    pub order_id: Option<String>,
    pub trade_id: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl Balance {
    pub fn new(
        exchange: String,
        account_id: String,
        available: Money,
        available_change: Money,
        currency: String,
        hold_value: Money,
        hold_change: Money,
        relation_event: String,
        relation_event_id: String,
        event_time: String,
        total: Money,
        symbol: Option<String>,
        order_id: Option<String>,
        trade_id: Option<String>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            account_id,
            available,
            available_change,
            currency,
            hold_value,
            hold_change,
            relation_event,
            relation_event_id,
            event_time,
            total,
            symbol,
            order_id,
            trade_id,
            updated_at,
        }
    }
}
