use crate::api::models::EventOrder;
use crate::core::error::AppResult;
use crate::repositories::EventOrderRepository;

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
}
