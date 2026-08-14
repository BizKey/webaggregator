use super::RepositoryResult;
use crate::api::models::Error;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ErrorRepository: Send + Sync {
    async fn get_errors(&self) -> RepositoryResult<Vec<Error>>;
}

pub struct PostgresErrorRepository {
    pool: PgPool,
}

impl PostgresErrorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ErrorRepository for PostgresErrorRepository {
    async fn get_errors(&self) -> RepositoryResult<Vec<Error>> {
        let errors = sqlx::query_as::<_, Error>(
            r#"
            SELECT exchange, msg, updated_at
            FROM errors
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(errors)
    }
}
