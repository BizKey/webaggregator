use crate::api::models::{
    PgConnection, PgStatStatements, PgStatTableSize, PgTableIndex, PgTableInfo,
};
use crate::core::error::AppResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ConnectionStatsRepository: Send + Sync {
    async fn get_connections(&self) -> AppResult<Vec<PgConnection>>;
}
#[async_trait]
pub trait TableStatsRepository: Send + Sync {
    async fn get_table_info(&self) -> AppResult<Vec<PgTableInfo>>;
    async fn get_table_indexes(&self) -> AppResult<Vec<PgTableIndex>>;
}

#[async_trait]
pub trait QueryStatsRepository: Send + Sync {
    async fn get_stat_statements(&self) -> AppResult<Vec<PgStatStatements>>;
}

#[async_trait]
pub trait TableSizeRepository: Send + Sync {
    async fn get_table_sizes(&self) -> AppResult<Vec<PgStatTableSize>>;
}

#[async_trait]
pub trait PgRepository:
    ConnectionStatsRepository + TableStatsRepository + QueryStatsRepository + TableSizeRepository
{
}

pub struct PostgresPgRepository {
    pool: PgPool,
}

impl PostgresPgRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConnectionStatsRepository for PostgresPgRepository {
    async fn get_connections(&self) -> AppResult<Vec<PgConnection>> {
        let connections = sqlx::query_as::<_, PgConnection>(
            r#"
            SELECT count(*) AS total_connections, 
                   count(*) FILTER (WHERE state = 'active') AS active_connections
            FROM pg_stat_activity;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(connections)
    }
}

#[async_trait]
impl TableStatsRepository for PostgresPgRepository {
    async fn get_table_info(&self) -> AppResult<Vec<PgTableInfo>> {
        let info = sqlx::query_as::<_, PgTableInfo>(
            r#"
            SELECT schemaname, relname, seq_scan, seq_tup_read, idx_scan, idx_tup_fetch, 
                   n_tup_ins, n_tup_upd, n_tup_del, n_live_tup, n_dead_tup
            FROM pg_stat_user_tables;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(info)
    }
    async fn get_table_indexes(&self) -> AppResult<Vec<PgTableIndex>> {
        let indexes = sqlx::query_as::<_, PgTableIndex>(
            r#"
            SELECT schemaname, relname, idx_scan, idx_tup_read, idx_tup_fetch
            FROM pg_stat_user_indexes;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(indexes)
    }
}

#[async_trait]
impl QueryStatsRepository for PostgresPgRepository {
    async fn get_stat_statements(&self) -> AppResult<Vec<PgStatStatements>> {
        let statements = sqlx::query_as::<_, PgStatStatements>(
            r#"
            SELECT query, calls, total_exec_time, mean_exec_time, rows
            FROM pg_stat_statements
            ORDER BY total_exec_time DESC LIMIT 100;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(statements)
    }
}

#[async_trait]
impl TableSizeRepository for PostgresPgRepository {
    async fn get_table_sizes(&self) -> AppResult<Vec<PgStatTableSize>> {
        let sizes = sqlx::query_as::<_, PgStatTableSize>(
            r#"
            SELECT schemaname, relname, 
                   pg_size_pretty(pg_total_relation_size(schemaname || '.' || relname)) AS total_size, 
                   pg_size_pretty(pg_relation_size(schemaname || '.' || relname)) AS table_size, 
                   pg_size_pretty(pg_indexes_size(schemaname || '.' || relname)) AS indexes_size
            FROM pg_stat_user_tables;
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sizes)
    }
}

#[async_trait]
impl PgRepository for PostgresPgRepository {}
