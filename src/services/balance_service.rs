use crate::api::models::Balance;
use crate::core::error::AppResult;
use crate::repositories::BalanceRepository;

pub struct BalanceService<R: BalanceRepository> {
    repo: R,
}

impl<R: BalanceRepository> BalanceService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_balances(&self, limit: i64) -> AppResult<Vec<Balance>> {
        self.repo.get_balances(limit).await.map_err(Into::into)
    }

    pub async fn clear_balances(&self) -> AppResult<u64> {
        self.repo.clear_balances().await.map_err(Into::into)
    }
}
