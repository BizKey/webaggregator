use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct PgConnectionDto {
    pub total_connections: i64,
    pub active_connections: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct PgTableInfoDto {
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

#[derive(Debug, Serialize, Clone)]
pub struct PgTableIndexDto {
    pub schemaname: String,
    pub relname: String,
    pub idx_scan: Option<i64>,
    pub idx_tup_read: Option<i64>,
    pub idx_tup_fetch: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PgStatStatementsDto {
    pub query: String,
    pub calls: Option<i64>,
    pub total_exec_time: Option<f64>,
    pub mean_exec_time: Option<f64>,
    pub rows: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PgStatTableSizeDto {
    pub schemaname: String,
    pub relname: String,
    pub total_size: String,
    pub table_size: String,
    pub indexes_size: String,
}
