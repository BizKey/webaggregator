use crate::domain::entities::Bot;
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait BotRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Bot>, RepositoryError>;
}
