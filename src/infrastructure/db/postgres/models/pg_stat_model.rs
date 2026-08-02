use crate::domain::repositories::*;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PgConnectionModel {
    pub total_connections: i64,
    pub active_connections: i64,
}

impl From<PgConnectionModel> for PgConnection {
    fn from(model: PgConnectionModel) -> Self {
        Self {
            total_connections: model.total_connections,
            active_connections: model.active_connections,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct PgTableInfoModel {
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

impl From<PgTableInfoModel> for PgTableInfo {
    fn from(model: PgTableInfoModel) -> Self {
        Self {
            schemaname: model.schemaname,
            relname: model.relname,
            seq_scan: model.seq_scan,
            seq_tup_read: model.seq_tup_read,
            idx_scan: model.idx_scan,
            idx_tup_fetch: model.idx_tup_fetch,
            n_tup_ins: model.n_tup_ins,
            n_tup_upd: model.n_tup_upd,
            n_tup_del: model.n_tup_del,
            n_live_tup: model.n_live_tup,
            n_dead_tup: model.n_dead_tup,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct PgTableIndexModel {
    pub schemaname: String,
    pub relname: String,
    pub idx_scan: Option<i64>,
    pub idx_tup_read: Option<i64>,
    pub idx_tup_fetch: Option<i64>,
}

impl From<PgTableIndexModel> for PgTableIndex {
    fn from(model: PgTableIndexModel) -> Self {
        Self {
            schemaname: model.schemaname,
            relname: model.relname,
            idx_scan: model.idx_scan,
            idx_tup_read: model.idx_tup_read,
            idx_tup_fetch: model.idx_tup_fetch,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct PgStatStatementsModel {
    pub query: String,
    pub calls: Option<i64>,
    pub total_exec_time: Option<f64>,
    pub mean_exec_time: Option<f64>,
    pub rows: Option<i64>,
}

impl From<PgStatStatementsModel> for PgStatStatements {
    fn from(model: PgStatStatementsModel) -> Self {
        Self {
            query: model.query,
            calls: model.calls,
            total_exec_time: model.total_exec_time,
            mean_exec_time: model.mean_exec_time,
            rows: model.rows,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct PgStatTableSizeModel {
    pub schemaname: String,
    pub relname: String,
    pub total_size: String,
    pub table_size: String,
    pub indexes_size: String,
}

impl From<PgStatTableSizeModel> for PgStatTableSize {
    fn from(model: PgStatTableSizeModel) -> Self {
        Self {
            schemaname: model.schemaname,
            relname: model.relname,
            total_size: model.total_size,
            table_size: model.table_size,
            indexes_size: model.indexes_size,
        }
    }
}
