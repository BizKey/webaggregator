use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub exchange: String,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgEvent {
    pub exchange: String,
    pub msg: Option<String>,
    pub code: Option<String>,
    pub borrow_size: Option<String>,
    pub client_oid: Option<String>,
    pub order_id: Option<String>,
    pub loan_apply_id: Option<String>,
    pub limit_rate: Option<f64>,
    pub reset_rate: Option<f64>,
    pub remaining_rate: Option<f64>,
    pub in_time: f64,
    pub out_time: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgSend {
    pub exchange: String,
    pub args_symbol: Option<String>,
    pub args_side: Option<String>,
    pub args_size: Option<String>,
    pub args_funds: Option<String>,
    pub args_price: Option<String>,
    pub args_time_in_force: Option<String>,
    pub args_type: Option<String>,
    pub args_auto_borrow: Option<bool>,
    pub args_auto_repay: Option<bool>,
    pub args_client_oid: Option<String>,
    pub args_order_id: Option<String>,
    pub updated_at: DateTime<Utc>,
}
