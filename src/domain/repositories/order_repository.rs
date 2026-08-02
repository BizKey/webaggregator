use crate::domain::entities::EventOrder;
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<EventOrder>, RepositoryError>;
}
