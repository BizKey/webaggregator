use crate::api::models::{PositionAsset, PositionDebt, PositionRatio};
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait PositionRepository: Send + Sync {
    async fn get_position_assets(&self) -> RepositoryResult<Vec<PositionAsset>>;
    async fn get_position_debts(&self) -> RepositoryResult<Vec<PositionDebt>>;
    async fn get_position_ratios(&self) -> RepositoryResult<Vec<PositionRatio>>;
}

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
    async fn get_position_assets(&self) -> RepositoryResult<Vec<PositionAsset>> {
        let positions = sqlx::query_as::<_, PositionAsset>(
            r#"
            SELECT exchange, asset_symbol, asset_total, asset_available, asset_hold, updated_at
            FROM positionasset
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(positions)
    }

    async fn get_position_debts(&self) -> RepositoryResult<Vec<PositionDebt>> {
        let positions = sqlx::query_as::<_, PositionDebt>(
            r#"
            SELECT exchange, debt_symbol, debt_value, updated_at
            FROM positiondebt
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(positions)
    }

    async fn get_position_ratios(&self) -> RepositoryResult<Vec<PositionRatio>> {
        let positions = sqlx::query_as::<_, PositionRatio>(
            r#"
            SELECT exchange, debt_ratio, total_asset, margin_coefficient_total_asset, 
                   total_debt, updated_at
            FROM positionratio
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(positions)
    }
}
