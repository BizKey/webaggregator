use crate::api::models::SendOrder;
use crate::core::error::AppResult;
use crate::repositories::SendOrderRepository;

pub struct SendOrderService<R: SendOrderRepository> {
    repo: R,
}

impl<R: SendOrderRepository> SendOrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_sendorders(&self) -> AppResult<Vec<SendOrder>> {
        self.repo.get_sendorders().await.map_err(Into::into)
    }
}
