use crate::domain::entities::Currency;
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait CurrencyRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Currency>, RepositoryError>;
}
