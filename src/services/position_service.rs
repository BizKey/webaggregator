use crate::api::models::{PositionAsset, PositionDebt, PositionRatio};
use crate::core::error::AppResult;
use crate::repositories::PositionRepository;

pub struct PositionService<R: PositionRepository> {
    repo: R,
}

impl<R: PositionRepository> PositionService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_position_assets(&self) -> AppResult<Vec<PositionAsset>> {
        self.repo.get_position_assets().await.map_err(Into::into)
    }

    pub async fn get_position_debts(&self) -> AppResult<Vec<PositionDebt>> {
        self.repo.get_position_debts().await.map_err(Into::into)
    }

    pub async fn get_position_ratios(&self) -> AppResult<Vec<PositionRatio>> {
        self.repo.get_position_ratios().await.map_err(Into::into)
    }
}
