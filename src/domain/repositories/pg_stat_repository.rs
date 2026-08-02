use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgConnection {
    pub total_connections: i64,
    pub active_connections: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgTableInfo {
    pub schemaname: String,
    pub relname: String,
    pub seq_scan: Option<i64>,
    pub seq_tup_read: Option<i64>,
    pub idx_scan: Option<i64>,
    pub idx_tup_fetch: Option<i64>,
    pub n_tup_ins: Option<i64>,
    pub n_tup_upd: Option<i64>,
    pub n_tup_del: Option<i64>,
    pub n_live_tup: Option<i64>,
    pub n_dead_tup: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgTableIndex {
    pub schemaname: String,
    pub relname: String,
    pub idx_scan: Option<i64>,
    pub idx_tup_read: Option<i64>,
    pub idx_tup_fetch: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgStatStatements {
    pub query: String,
    pub calls: Option<i64>,
    pub total_exec_time: Option<f64>,
    pub mean_exec_time: Option<f64>,
    pub rows: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgStatTableSize {
    pub schemaname: String,
    pub relname: String,
    pub total_size: String,
    pub table_size: String,
    pub indexes_size: String,
}

#[async_trait]
pub trait PgStatRepository: Send + Sync {
    async fn get_connections(&self) -> Result<Vec<PgConnection>, RepositoryError>;
    async fn get_table_info(&self) -> Result<Vec<PgTableInfo>, RepositoryError>;
    async fn get_table_index(&self) -> Result<Vec<PgTableIndex>, RepositoryError>;
    async fn get_statements(&self) -> Result<Vec<PgStatStatements>, RepositoryError>;
    async fn get_table_sizes(&self) -> Result<Vec<PgStatTableSize>, RepositoryError>;
}
