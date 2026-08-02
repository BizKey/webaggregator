use crate::domain::entities::Ticker;
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait TickerRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Ticker>, RepositoryError>;
}
