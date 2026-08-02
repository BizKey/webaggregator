use crate::domain::entities::Bot;
use crate::domain::repositories::{BotRepository, RepositoryError};
use crate::infrastructure::db::postgres::models::BotModel;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
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
    async fn find_all(&self) -> Result<Vec<Bot>, RepositoryError> {
        let models = sqlx::query_as::<_, BotModel>(
            r#"
            SELECT exchange, entry_client_oid, exit_tp_order_id, exit_tp_client_oid,
                   exit_sl_order_id, exit_sl_client_oid, symbol, balance, updated_at
            FROM bots
            ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(Bot::from).collect())
    }
}
