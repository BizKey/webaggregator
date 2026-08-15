use super::RepositoryResult;
use crate::api::models::MsgSend;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait MsgSendRepository: Send + Sync {
    async fn get_msgsends(&self) -> RepositoryResult<Vec<MsgSend>>;
}

pub struct PostgresMsgSendRepository {
    pool: PgPool,
}

impl PostgresMsgSendRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MsgSendRepository for PostgresMsgSendRepository {
    async fn get_msgsends(&self) -> RepositoryResult<Vec<MsgSend>> {
        let msgsends = sqlx::query_as::<_, MsgSend>(
            r#"
            SELECT exchange, args_symbol, args_side, args_size, args_funds, 
                   args_price, args_time_in_force, args_type, args_auto_borrow, 
                   args_auto_repay, args_client_oid, args_order_id, updated_at
            FROM msgsend
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(msgsends)
    }
}
