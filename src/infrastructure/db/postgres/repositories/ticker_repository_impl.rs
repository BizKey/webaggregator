use crate::domain::entities::Ticker;
use crate::domain::repositories::{RepositoryError, TickerRepository};
use crate::infrastructure::db::postgres::models::TickerModel;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
pub struct PostgresTickerRepository {
    pool: PgPool,
}

impl PostgresTickerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TickerRepository for PostgresTickerRepository {
    async fn find_all(&self) -> Result<Vec<Ticker>, RepositoryError> {
        let models = sqlx::query_as::<_, TickerModel>(
            r#"
            SELECT exchange, symbol, symbol_name, taker_fee_rate, maker_fee_rate, 
                   taker_coefficient, maker_coefficient, updated_at
            FROM ticker
            ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(Ticker::from).collect())
    }
}
