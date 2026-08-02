use crate::domain::entities::Balance;
use crate::domain::repositories::{BalanceRepository, RepositoryError};
use crate::infrastructure::db::postgres::models::BalanceModel;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
pub struct PostgresBalanceRepository {
    pool: PgPool,
}

impl PostgresBalanceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BalanceRepository for PostgresBalanceRepository {
    async fn find_all(&self) -> Result<Vec<Balance>, RepositoryError> {
        let models = sqlx::query_as::<_, BalanceModel>(
            r#"
            SELECT exchange, account_id, available, available_change, currency, 
                   hold_value, hold_change, relation_event, relation_event_id, 
                   event_time, total, symbol, order_id, trade_id, updated_at
            FROM balance
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(Balance::from).collect())
    }
}
