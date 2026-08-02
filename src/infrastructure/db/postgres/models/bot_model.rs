use crate::domain::entities::Bot;
use crate::domain::value_objects::{Exchange, SymbolName};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct BotModel {
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

impl From<BotModel> for Bot {
    fn from(model: BotModel) -> Self {
        Self {
            exchange: model.exchange.map(Exchange::new),
            entry_client_oid: model.entry_client_oid,
            exit_tp_order_id: model.exit_tp_order_id,
            exit_tp_client_oid: model.exit_tp_client_oid,
            exit_sl_order_id: model.exit_sl_order_id,
            exit_sl_client_oid: model.exit_sl_client_oid,
            symbol: model.symbol.map(SymbolName::new),
            balance: model.balance,
            updated_at: model.updated_at,
        }
    }
}
