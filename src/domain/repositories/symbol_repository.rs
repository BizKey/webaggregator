use crate::domain::entities::Symbol;
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait SymbolRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Symbol>, RepositoryError>;
}
