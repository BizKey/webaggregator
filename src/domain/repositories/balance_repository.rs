use crate::domain::entities::Balance;
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait BalanceRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Balance>, RepositoryError>;
}
