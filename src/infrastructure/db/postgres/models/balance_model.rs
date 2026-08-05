use crate::domain::entities::Balance;
use crate::domain::value_objects::{Exchange, Money, SymbolName};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct BalanceModel {
    pub exchange: String,
    pub account_id: String,
    pub available: String,
    pub available_change: String,
    pub currency: String,
    pub hold_value: String,
    pub hold_change: String,
    pub relation_event: String,
    pub relation_event_id: String,
    pub event_time: String,
    pub total: String,
    pub symbol: Option<String>,
    pub order_id: Option<String>,
    pub trade_id: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<BalanceModel> for Balance {
    fn from(model: BalanceModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            account_id: model.account_id,
            available: Money::new(model.available.parse().unwrap_or(0.0)),
            available_change: Money::new(model.available_change.parse().unwrap_or(0.0)),
            currency: model.currency,
            hold_value: Money::new(model.hold_value.parse().unwrap_or(0.0)),
            hold_change: Money::new(model.hold_change.parse().unwrap_or(0.0)),
            relation_event: model.relation_event,
            relation_event_id: model.relation_event_id,
            event_time: model.event_time,
            total: Money::new(model.total.parse().unwrap_or(0.0)),
            symbol: model.symbol.map(SymbolName::new),
            order_id: model.order_id,
            trade_id: model.trade_id,
            updated_at: model.updated_at,
        }
    }
}
