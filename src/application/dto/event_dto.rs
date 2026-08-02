use crate::domain::entities::{Event, MsgEvent, MsgSend};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct EventDto {
    pub exchange: String,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}

impl From<Event> for EventDto {
    fn from(event: Event) -> Self {
        Self {
            exchange: event.exchange.as_str().to_string(),
            msg: event.msg,
            updated_at: event.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct MsgEventDto {
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

impl From<MsgEvent> for MsgEventDto {
    fn from(event: MsgEvent) -> Self {
        Self {
            exchange: event.exchange.as_str().to_string(),
            msg: event.msg,
            code: event.code,
            borrow_size: event.borrow_size,
            client_oid: event.client_oid,
            order_id: event.order_id,
            loan_apply_id: event.loan_apply_id,
            limit_rate: event.limit_rate,
            reset_rate: event.reset_rate,
            remaining_rate: event.remaining_rate,
            in_time: event.in_time,
            out_time: event.out_time,
            updated_at: event.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct MsgSendDto {
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

impl From<MsgSend> for MsgSendDto {
    fn from(event: MsgSend) -> Self {
        Self {
            exchange: event.exchange.as_str().to_string(),
            args_symbol: event.args_symbol,
            args_side: event.args_side,
            args_size: event.args_size,
            args_funds: event.args_funds,
            args_price: event.args_price,
            args_time_in_force: event.args_time_in_force,
            args_type: event.args_type,
            args_auto_borrow: event.args_auto_borrow,
            args_auto_repay: event.args_auto_repay,
            args_client_oid: event.args_client_oid,
            args_order_id: event.args_order_id,
            updated_at: event.updated_at,
        }
    }
}
