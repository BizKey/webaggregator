use crate::domain::entities::{Event, MsgEvent, MsgSend};
use crate::domain::repositories::{EventRepository, RepositoryError};
use crate::infrastructure::db::postgres::models::{EventModel, MsgEventModel, MsgSendModel};
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
pub struct PostgresEventRepository {
    pool: PgPool,
}

impl PostgresEventRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventRepository for PostgresEventRepository {
    async fn find_events(&self) -> Result<Vec<Event>, RepositoryError> {
        let models = sqlx::query_as::<_, EventModel>(
            r#"
            SELECT exchange, msg, updated_at
            FROM events
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(Event::from).collect())
    }

    async fn find_msg_events(&self) -> Result<Vec<MsgEvent>, RepositoryError> {
        let models = sqlx::query_as::<_, MsgEventModel>(
            r#"
            SELECT exchange, msg, code, borrow_size, client_oid, order_id, loan_apply_id,
                   limit_rate, reset_rate, remaining_rate, in_time, out_time, updated_at
            FROM msgevent
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(MsgEvent::from).collect())
    }

    async fn find_msg_sends(&self) -> Result<Vec<MsgSend>, RepositoryError> {
        let models = sqlx::query_as::<_, MsgSendModel>(
            r#"
            SELECT exchange, args_symbol, args_side, args_size, args_funds, args_price,
                   args_time_in_force, args_type, args_auto_borrow, args_auto_repay,
                   args_client_oid, args_order_id, updated_at
            FROM msgsend
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(MsgSend::from).collect())
    }
}
