use crate::api::models::SendOrder;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait SendOrderRepository: Send + Sync {
    async fn get_sendorders(&self) -> RepositoryResult<Vec<SendOrder>>;
}

pub struct PostgresSendOrderRepository {
    pool: PgPool,
}

impl PostgresSendOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SendOrderRepository for PostgresSendOrderRepository {
    async fn get_sendorders(&self) -> RepositoryResult<Vec<SendOrder>> {
        let sendorders = sqlx::query_as::<_, SendOrder>(
            r#"
            SELECT exchange, client_oid, side, symbol, order_type, size, funds, price, 
                time_in_force, auto_borrow, auto_repay, order_id, updated_at
            FROM sendorders
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sendorders)
    }
}
