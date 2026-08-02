use crate::domain::entities::EventOrder;
use crate::domain::repositories::{OrderRepository, RepositoryError};
use crate::infrastructure::db::postgres::models::EventOrderModel;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn find_all(&self) -> Result<Vec<EventOrder>, RepositoryError> {
        let models = sqlx::query_as::<_, EventOrderModel>(
            r#"
            SELECT exchange, status, type_, symbol, side, order_type, fee_type, liquidity,
                   price, order_id, client_oid, trade_id, origin_size, size, filled_size,
                   match_size, match_price, canceled_size, old_size, remain_size, remain_funds,
                   order_time, ts, updated_at
            FROM orderevent
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(EventOrder::from).collect())
    }
}
