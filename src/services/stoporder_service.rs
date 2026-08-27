use crate::api::models::StopOrder;
use crate::core::error::AppResult;
use crate::repositories::StopOrderRepository;

pub struct StopOrderService<R: StopOrderRepository> {
    repo: R,
}

impl<R: StopOrderRepository> StopOrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_stoporders(&self) -> AppResult<Vec<StopOrder>> {
        self.repo.get_stoporders().await.map_err(Into::into)
    }
}
