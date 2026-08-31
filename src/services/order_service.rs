use crate::api::models::EventOrder;
use crate::core::error::AppResult;

use crate::repositories::{EventOrderRepository, TradeWithStop};
pub struct OrderService<R: EventOrderRepository> {
    repo: R,
}

impl<R: EventOrderRepository> OrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_event_orders(&self) -> AppResult<Vec<EventOrder>> {
        self.repo.get_event_orders().await.map_err(Into::into)
    }
    pub async fn get_trades_with_stops(
        &self,
        symbol: &str,
        limit: i64,
    ) -> AppResult<Vec<TradeWithStop>> {
        self.repo
            .get_trades_with_stops(symbol, limit)
            .await
            .map_err(Into::into)
    }
}
