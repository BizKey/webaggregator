use super::RepositoryResult;
use crate::api::models::Bot;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait BotRepository: Send + Sync {
    async fn get_bots(&self) -> RepositoryResult<Vec<Bot>>;
}

pub struct PostgresBotRepository {
    pool: PgPool,
}

impl PostgresBotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BotRepository for PostgresBotRepository {
    async fn get_bots(&self) -> RepositoryResult<Vec<Bot>> {
        let bots = sqlx::query_as::<_, Bot>(
            r#"
            SELECT exchange, entry_price, entry_client_oid, exit_tp_price, 
                   exit_tp_order_id, exit_tp_client_oid, exit_sl_price, 
                   exit_sl_order_id, exit_sl_client_oid, symbol, balance, updated_at
            FROM bots
            ORDER BY updated_at DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(bots)
    }
}
