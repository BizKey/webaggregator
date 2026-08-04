use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOrder {
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
