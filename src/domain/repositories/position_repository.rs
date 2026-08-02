use crate::domain::entities::{PositionAsset, PositionDebt, PositionRatio};
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait PositionRepository: Send + Sync {
    async fn find_assets(&self) -> Result<Vec<PositionAsset>, RepositoryError>;
    async fn find_debts(&self) -> Result<Vec<PositionDebt>, RepositoryError>;
    async fn find_ratios(&self) -> Result<Vec<PositionRatio>, RepositoryError>;
}
