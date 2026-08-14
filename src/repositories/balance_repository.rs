use super::RepositoryResult;
use crate::api::models::Balance;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait BalanceRepository: Send + Sync {
    async fn get_balances(&self, limit: i64) -> RepositoryResult<Vec<Balance>>;
}

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
    async fn get_balances(&self, limit: i64) -> RepositoryResult<Vec<Balance>> {
        let balances = sqlx::query_as::<_, Balance>(
            r#"
            SELECT exchange, account_id, available, available_change, currency, 
                   hold_value, hold_change, relation_event, relation_event_id, 
                   event_time, total, symbol, order_id, trade_id, updated_at
            FROM balance
            ORDER BY updated_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(balances)
    }
}
