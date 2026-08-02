use crate::application::dto::EventOrderDto;
use crate::application::services::ServiceError;
use crate::domain::repositories::OrderRepository;
#[derive(Clone)]
pub struct OrderService<R: OrderRepository> {
    repository: R,
}

impl<R: OrderRepository> OrderService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_orders(&self) -> Result<Vec<EventOrderDto>, ServiceError> {
        let orders = self
            .repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(orders.into_iter().map(EventOrderDto::from).collect())
    }
}
