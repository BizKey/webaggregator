use crate::domain::entities::EventOrder;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct EventOrderModel {
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

impl From<EventOrderModel> for EventOrder {
    fn from(model: EventOrderModel) -> Self {
        Self {
            exchange: model.exchange,
            status: model.status,
            type_: model.type_,
            symbol: model.symbol,
            side: model.side,
            order_type: model.order_type,
            fee_type: model.fee_type,
            liquidity: model.liquidity,
            price: model.price,
            order_id: model.order_id,
            client_oid: model.client_oid,
            trade_id: model.trade_id,
            origin_size: model.origin_size,
            size: model.size,
            filled_size: model.filled_size,
            match_size: model.match_size,
            match_price: model.match_price,
            canceled_size: model.canceled_size,
            old_size: model.old_size,
            remain_size: model.remain_size,
            remain_funds: model.remain_funds,
            order_time: model.order_time,
            ts: model.ts,
            updated_at: model.updated_at,
        }
    }
}
