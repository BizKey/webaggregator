use crate::domain::entities::Error;
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait ErrorRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Error>, RepositoryError>;
}
