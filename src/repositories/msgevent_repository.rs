use crate::api::models::MsgEvent;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait MsgEventRepository: Send + Sync {
    async fn get_msgevents(&self) -> RepositoryResult<Vec<MsgEvent>>;
}

pub struct PostgresMsgEventRepository {
    pool: PgPool,
}

impl PostgresMsgEventRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MsgEventRepository for PostgresMsgEventRepository {
    async fn get_msgevents(&self) -> RepositoryResult<Vec<MsgEvent>> {
        let msgevents = sqlx::query_as::<_, MsgEvent>(
            r#"
            SELECT exchange, msg, code, borrow_size, client_oid, order_id, 
                   loan_apply_id, limit_rate, reset_rate, remaining_rate, 
                   in_time, out_time, updated_at
            FROM msgevent
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(msgevents)
    }
}
