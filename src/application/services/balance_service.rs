use crate::application::dto::BalanceDto;
use crate::application::services::ServiceError;
use crate::domain::repositories::BalanceRepository;
#[derive(Clone)]
pub struct BalanceService<R: BalanceRepository> {
    repository: R,
}

impl<R: BalanceRepository> BalanceService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_balances(&self) -> Result<Vec<BalanceDto>, ServiceError> {
        let balances = self
            .repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(balances.into_iter().map(BalanceDto::from).collect())
    }
}
