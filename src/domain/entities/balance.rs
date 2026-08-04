use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
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
