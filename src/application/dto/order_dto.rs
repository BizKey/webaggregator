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
    pub price: Option<String>,
    pub order_id: String,
    pub client_oid: Option<String>,
    pub trade_id: Option<String>,
    pub origin_size: Option<String>,
    pub size: Option<String>,
    pub filled_size: Option<String>,
    pub match_size: Option<String>,
    pub match_price: Option<String>,
    pub canceled_size: Option<String>,
    pub old_size: Option<String>,
    pub remain_size: Option<String>,
    pub remain_funds: Option<String>,
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
            price: order.price,
            order_id: order.order_id,
            client_oid: order.client_oid,
            trade_id: order.trade_id,
            origin_size: order.origin_size,
            size: order.size,
            filled_size: order.filled_size,
            match_size: order.match_size,
            match_price: order.match_price,
            canceled_size: order.canceled_size,
            old_size: order.old_size,
            remain_size: order.remain_size,
            remain_funds: order.remain_funds,
            order_time: order.order_time,
            ts: order.ts,
            updated_at: order.updated_at,
        }
    }
}
