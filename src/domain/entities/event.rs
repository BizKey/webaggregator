use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub exchange: String,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}

impl Event {
    pub fn new(exchange: String, msg: String, updated_at: DateTime<Utc>) -> Self {
        Self {
            exchange,
            msg,
            updated_at,
        }
    }
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

impl MsgEvent {
    pub fn new(
        exchange: String,
        msg: Option<String>,
        code: Option<String>,
        borrow_size: Option<String>,
        client_oid: Option<String>,
        order_id: Option<String>,
        loan_apply_id: Option<String>,
        limit_rate: Option<f64>,
        reset_rate: Option<f64>,
        remaining_rate: Option<f64>,
        in_time: f64,
        out_time: f64,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            msg,
            code,
            borrow_size,
            client_oid,
            order_id,
            loan_apply_id,
            limit_rate,
            reset_rate,
            remaining_rate,
            in_time,
            out_time,
            updated_at,
        }
    }
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

impl MsgSend {
    pub fn new(
        exchange: String,
        args_symbol: Option<String>,
        args_side: Option<String>,
        args_size: Option<String>,
        args_funds: Option<String>,
        args_price: Option<String>,
        args_time_in_force: Option<String>,
        args_type: Option<String>,
        args_auto_borrow: Option<bool>,
        args_auto_repay: Option<bool>,
        args_client_oid: Option<String>,
        args_order_id: Option<String>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            args_symbol,
            args_side,
            args_size,
            args_funds,
            args_price,
            args_time_in_force,
            args_type,
            args_auto_borrow,
            args_auto_repay,
            args_client_oid,
            args_order_id,
            updated_at,
        }
    }
}
