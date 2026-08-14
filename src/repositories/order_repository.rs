use super::RepositoryResult;
use crate::api::models::EventOrder;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait EventOrderRepository: Send + Sync {
    async fn get_event_orders(&self) -> RepositoryResult<Vec<EventOrder>>;
}

pub struct PostgresEventOrderRepository {
    pool: PgPool,
}

impl PostgresEventOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventOrderRepository for PostgresEventOrderRepository {
    async fn get_event_orders(&self) -> RepositoryResult<Vec<EventOrder>> {
        let event_orders = sqlx::query_as::<_, EventOrder>(
            r#"
            SELECT exchange, status, type_, symbol, side, order_type, fee_type, 
                   liquidity, price, order_id, client_oid, trade_id, origin_size, 
                   size, filled_size, match_size, match_price, canceled_size, 
                   old_size, remain_size, remain_funds, order_time, ts, updated_at
            FROM orderevent
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(event_orders)
    }
}
