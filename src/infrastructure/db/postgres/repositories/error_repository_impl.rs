use crate::domain::entities::Error;
use crate::domain::repositories::{ErrorRepository, RepositoryError};
use crate::infrastructure::db::postgres::models::ErrorModel;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
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
    async fn find_all(&self) -> Result<Vec<Error>, RepositoryError> {
        let models = sqlx::query_as::<_, ErrorModel>(
            r#"
            SELECT exchange, msg, updated_at
            FROM errors
            ORDER BY updated_at DESC
            LIMIT 1000
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(Error::from).collect())
    }
}
