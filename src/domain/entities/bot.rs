use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    pub exchange: Option<String>,
    pub entry_client_oid: Option<String>,
    pub exit_tp_order_id: Option<String>,
    pub exit_tp_client_oid: Option<String>,
    pub exit_sl_order_id: Option<String>,
    pub exit_sl_client_oid: Option<String>,
    pub symbol: Option<String>,
    pub balance: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl Bot {
    pub fn new(
        exchange: Option<String>,
        entry_client_oid: Option<String>,
        exit_tp_order_id: Option<String>,
        exit_tp_client_oid: Option<String>,
        exit_sl_order_id: Option<String>,
        exit_sl_client_oid: Option<String>,
        symbol: Option<String>,
        balance: Option<String>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            entry_client_oid,
            exit_tp_order_id,
            exit_tp_client_oid,
            exit_sl_order_id,
            exit_sl_client_oid,
            symbol,
            balance,
            updated_at,
        }
    }
}
