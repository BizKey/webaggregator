use crate::api::models::StopOrder;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait StopOrderRepository: Send + Sync {
    async fn get_stoporders(&self) -> RepositoryResult<Vec<StopOrder>>;
}

pub struct PostgresStopOrderRepository {
    pool: PgPool,
}

impl PostgresStopOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StopOrderRepository for PostgresStopOrderRepository {
    async fn get_stoporders(&self) -> RepositoryResult<Vec<StopOrder>> {
        let stoporders = sqlx::query_as::<_, StopOrder>(
            r#"
            SELECT exchange, client_oid, side, symbol, order_type, stop_type,
                   stop_price, size, funds, time_in_force, auto_borrow, auto_repay,
                   is_isolated, updated_at
            FROM stoporders
            ORDER BY updated_at DESC
            LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stoporders)
    }
}
