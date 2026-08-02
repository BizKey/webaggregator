use crate::domain::value_objects::{Exchange, SymbolName};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    pub exchange: Option<Exchange>,
    pub entry_client_oid: Option<String>,
    pub exit_tp_order_id: Option<String>,
    pub exit_tp_client_oid: Option<String>,
    pub exit_sl_order_id: Option<String>,
    pub exit_sl_client_oid: Option<String>,
    pub symbol: Option<SymbolName>,
    pub balance: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl Bot {
    pub fn new(
        exchange: Option<Exchange>,
        entry_client_oid: Option<String>,
        exit_tp_order_id: Option<String>,
        exit_tp_client_oid: Option<String>,
        exit_sl_order_id: Option<String>,
        exit_sl_client_oid: Option<String>,
        symbol: Option<SymbolName>,
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
