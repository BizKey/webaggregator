use crate::application::dto::*;
use crate::application::services::ServiceError;
use crate::domain::repositories::PgStatRepository;
#[derive(Clone)]
pub struct PgStatService<R: PgStatRepository> {
    repository: R,
}

impl<R: PgStatRepository> PgStatService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_connections(&self) -> Result<Vec<PgConnectionDto>, ServiceError> {
        let conns = self
            .repository
            .get_connections()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(conns
            .into_iter()
            .map(|c| PgConnectionDto {
                total_connections: c.total_connections,
                active_connections: c.active_connections,
            })
            .collect())
    }

    pub async fn get_table_info(&self) -> Result<Vec<PgTableInfoDto>, ServiceError> {
        let info = self
            .repository
            .get_table_info()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(info
            .into_iter()
            .map(|i| PgTableInfoDto {
                schemaname: i.schemaname,
                relname: i.relname,
                seq_scan: i.seq_scan,
                seq_tup_read: i.seq_tup_read,
                idx_scan: i.idx_scan,
                idx_tup_fetch: i.idx_tup_fetch,
                n_tup_ins: i.n_tup_ins,
                n_tup_upd: i.n_tup_upd,
                n_tup_del: i.n_tup_del,
                n_live_tup: i.n_live_tup,
                n_dead_tup: i.n_dead_tup,
            })
            .collect())
    }

    pub async fn get_table_index(&self) -> Result<Vec<PgTableIndexDto>, ServiceError> {
        let idx = self
            .repository
            .get_table_index()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(idx
            .into_iter()
            .map(|i| PgTableIndexDto {
                schemaname: i.schemaname,
                relname: i.relname,
                idx_scan: i.idx_scan,
                idx_tup_read: i.idx_tup_read,
                idx_tup_fetch: i.idx_tup_fetch,
            })
            .collect())
    }

    pub async fn get_statements(&self) -> Result<Vec<PgStatStatementsDto>, ServiceError> {
        let stmts = self
            .repository
            .get_statements()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(stmts
            .into_iter()
            .map(|s| PgStatStatementsDto {
                query: s.query,
                calls: s.calls,
                total_exec_time: s.total_exec_time,
                mean_exec_time: s.mean_exec_time,
                rows: s.rows,
            })
            .collect())
    }

    pub async fn get_table_sizes(&self) -> Result<Vec<PgStatTableSizeDto>, ServiceError> {
        let sizes = self
            .repository
            .get_table_sizes()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(sizes
            .into_iter()
            .map(|s| PgStatTableSizeDto {
                schemaname: s.schemaname,
                relname: s.relname,
                total_size: s.total_size,
                table_size: s.table_size,
                indexes_size: s.indexes_size,
            })
            .collect())
    }
}
