use crate::domain::repositories::{
    PgConnection, PgStatRepository, PgStatStatements, PgStatTableSize, PgTableIndex, PgTableInfo,
    RepositoryError,
};
use crate::infrastructure::db::postgres::models::*;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
pub struct PostgresPgStatRepository {
    pool: PgPool,
}

impl PostgresPgStatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PgStatRepository for PostgresPgStatRepository {
    async fn get_connections(&self) -> Result<Vec<PgConnection>, RepositoryError> {
        let models = sqlx::query_as::<_, PgConnectionModel>(
            r#"
            SELECT count(*) AS total_connections, count(*)
            FILTER (WHERE state = 'active') AS active_connections
            FROM pg_stat_activity
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PgConnection::from).collect())
    }

    async fn get_table_info(&self) -> Result<Vec<PgTableInfo>, RepositoryError> {
        let models = sqlx::query_as::<_, PgTableInfoModel>(
            r#"
            SELECT schemaname, relname, seq_scan, seq_tup_read, idx_scan, idx_tup_fetch,
                   n_tup_ins, n_tup_upd, n_tup_del, n_live_tup, n_dead_tup
            FROM pg_stat_user_tables
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PgTableInfo::from).collect())
    }

    async fn get_table_index(&self) -> Result<Vec<PgTableIndex>, RepositoryError> {
        let models = sqlx::query_as::<_, PgTableIndexModel>(
            r#"
            SELECT schemaname, relname, idx_scan, idx_tup_read, idx_tup_fetch
            FROM pg_stat_user_indexes
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PgTableIndex::from).collect())
    }

    async fn get_statements(&self) -> Result<Vec<PgStatStatements>, RepositoryError> {
        let models = sqlx::query_as::<_, PgStatStatementsModel>(
            r#"
            SELECT query, calls, total_exec_time, mean_exec_time, rows
            FROM pg_stat_statements
            ORDER BY total_exec_time DESC
            LIMIT 100
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PgStatStatements::from).collect())
    }

    async fn get_table_sizes(&self) -> Result<Vec<PgStatTableSize>, RepositoryError> {
        let models = sqlx::query_as::<_, PgStatTableSizeModel>(
            r#"
            SELECT schemaname, relname, 
                   pg_size_pretty(pg_total_relation_size(schemaname || '.' || relname)) AS total_size,
                   pg_size_pretty(pg_relation_size(schemaname || '.' || relname)) AS table_size,
                   pg_size_pretty(pg_indexes_size(schemaname || '.' || relname)) AS indexes_size
            FROM pg_stat_user_tables
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(PgStatTableSize::from).collect())
    }
}
