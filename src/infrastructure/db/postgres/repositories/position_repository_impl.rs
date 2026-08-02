use crate::domain::entities::{PositionAsset, PositionDebt, PositionRatio};
use crate::domain::repositories::{PositionRepository, RepositoryError};
use crate::infrastructure::db::postgres::models::{
    PositionAssetModel, PositionDebtModel, PositionRatioModel,
};
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
pub struct PostgresPositionRepository {
    pool: PgPool,
}

impl PostgresPositionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PositionRepository for PostgresPositionRepository {
    async fn find_assets(&self) -> Result<Vec<PositionAsset>, RepositoryError> {
        let models = sqlx::query_as::<_, PositionAssetModel>(
            r#"
            SELECT exchange, asset_symbol, asset_total, asset_available, asset_hold, updated_at
            FROM positionasset
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PositionAsset::from).collect())
    }

    async fn find_debts(&self) -> Result<Vec<PositionDebt>, RepositoryError> {
        let models = sqlx::query_as::<_, PositionDebtModel>(
            r#"
            SELECT exchange, debt_symbol, debt_value, updated_at
            FROM positiondebt
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PositionDebt::from).collect())
    }

    async fn find_ratios(&self) -> Result<Vec<PositionRatio>, RepositoryError> {
        let models = sqlx::query_as::<_, PositionRatioModel>(
            r#"
            SELECT exchange, debt_ratio, total_asset, margin_coefficient_total_asset, total_debt, updated_at
            FROM positionratio
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PositionRatio::from).collect())
    }
}
