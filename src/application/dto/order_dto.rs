use crate::domain::entities::EventOrder;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct EventOrderDto {
    pub exchange: String,
    pub status: String,
    pub type_: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub fee_type: Option<String>,
    pub liquidity: Option<String>,
    pub price: Option<f64>,
    pub order_id: String,
    pub client_oid: Option<String>,
    pub trade_id: Option<String>,
    pub origin_size: Option<f64>,
    pub size: Option<f64>,
    pub filled_size: Option<f64>,
    pub match_size: Option<f64>,
    pub match_price: Option<f64>,
    pub canceled_size: Option<f64>,
    pub old_size: Option<f64>,
    pub remain_size: Option<f64>,
    pub remain_funds: Option<f64>,
    pub order_time: i64,
    pub ts: i64,
    pub updated_at: DateTime<Utc>,
}

impl From<EventOrder> for EventOrderDto {
    fn from(order: EventOrder) -> Self {
        Self {
            exchange: order.exchange.as_str().to_string(),
            status: order.status.to_string(),
            type_: order.type_,
            symbol: order.symbol.as_str().to_string(),
            side: order.side.to_string(),
            order_type: order.order_type.to_string(),
            fee_type: order.fee_type,
            liquidity: order.liquidity,
            price: order.price.map(|m| m.value()),
            order_id: order.order_id,
            client_oid: order.client_oid,
            trade_id: order.trade_id,
            origin_size: order.origin_size.map(|m| m.value()),
            size: order.size.map(|m| m.value()),
            filled_size: order.filled_size.map(|m| m.value()),
            match_size: order.match_size.map(|m| m.value()),
            match_price: order.match_price.map(|m| m.value()),
            canceled_size: order.canceled_size.map(|m| m.value()),
            old_size: order.old_size.map(|m| m.value()),
            remain_size: order.remain_size.map(|m| m.value()),
            remain_funds: order.remain_funds.map(|m| m.value()),
            order_time: order.order_time,
            ts: order.ts,
            updated_at: order.updated_at,
        }
    }
}
