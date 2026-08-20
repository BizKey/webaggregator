use crate::api::models::Event;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn get_events(&self) -> RepositoryResult<Vec<Event>>;
}

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
    async fn get_events(&self) -> RepositoryResult<Vec<Event>> {
        let events = sqlx::query_as::<_, Event>(
            r#"
            SELECT exchange, msg, updated_at
            FROM events
            ORDER BY updated_at
            DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(events)
    }
}
