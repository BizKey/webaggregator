use crate::domain::entities::{Event, MsgEvent, MsgSend};
use crate::domain::value_objects::Exchange;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct EventModel {
    pub exchange: String,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}

impl From<EventModel> for Event {
    fn from(model: EventModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            msg: model.msg,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct MsgEventModel {
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

impl From<MsgEventModel> for MsgEvent {
    fn from(model: MsgEventModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            msg: model.msg,
            code: model.code,
            borrow_size: model.borrow_size,
            client_oid: model.client_oid,
            order_id: model.order_id,
            loan_apply_id: model.loan_apply_id,
            limit_rate: model.limit_rate,
            reset_rate: model.reset_rate,
            remaining_rate: model.remaining_rate,
            in_time: model.in_time,
            out_time: model.out_time,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct MsgSendModel {
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

impl From<MsgSendModel> for MsgSend {
    fn from(model: MsgSendModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            args_symbol: model.args_symbol,
            args_side: model.args_side,
            args_size: model.args_size,
            args_funds: model.args_funds,
            args_price: model.args_price,
            args_time_in_force: model.args_time_in_force,
            args_type: model.args_type,
            args_auto_borrow: model.args_auto_borrow,
            args_auto_repay: model.args_auto_repay,
            args_client_oid: model.args_client_oid,
            args_order_id: model.args_order_id,
            updated_at: model.updated_at,
        }
    }
}
