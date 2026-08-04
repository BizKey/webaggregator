use crate::domain::entities::Bot;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct BotDto {
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

impl From<Bot> for BotDto {
    fn from(bot: Bot) -> Self {
        Self {
            exchange: bot.exchange,
            entry_client_oid: bot.entry_client_oid,
            exit_tp_order_id: bot.exit_tp_order_id,
            exit_tp_client_oid: bot.exit_tp_client_oid,
            exit_sl_order_id: bot.exit_sl_order_id,
            exit_sl_client_oid: bot.exit_sl_client_oid,
            symbol: bot.symbol,
            balance: bot.balance,
            updated_at: bot.updated_at,
        }
    }
}
