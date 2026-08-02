use crate::application::dto::{PositionAssetDto, PositionDebtDto, PositionRatioDto};
use crate::application::services::ServiceError;
use crate::domain::repositories::PositionRepository;
#[derive(Clone)]
pub struct PositionService<R: PositionRepository> {
    repository: R,
}

impl<R: PositionRepository> PositionService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_assets(&self) -> Result<Vec<PositionAssetDto>, ServiceError> {
        let assets = self
            .repository
            .find_assets()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(assets.into_iter().map(PositionAssetDto::from).collect())
    }

    pub async fn get_debts(&self) -> Result<Vec<PositionDebtDto>, ServiceError> {
        let debts = self
            .repository
            .find_debts()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(debts.into_iter().map(PositionDebtDto::from).collect())
    }

    pub async fn get_ratios(&self) -> Result<Vec<PositionRatioDto>, ServiceError> {
        let ratios = self
            .repository
            .find_ratios()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(ratios.into_iter().map(PositionRatioDto::from).collect())
    }
}
