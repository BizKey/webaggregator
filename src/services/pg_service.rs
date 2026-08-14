use crate::api::models::{
    PgConnection, PgStatStatements, PgStatTableSize, PgTableIndex, PgTableInfo,
};
use crate::core::error::AppResult;
use crate::repositories::{
    ConnectionStatsRepository, QueryStatsRepository, TableSizeRepository, TableStatsRepository,
};

pub struct PgService<R>
where
    R: ConnectionStatsRepository
        + TableStatsRepository
        + QueryStatsRepository
        + TableSizeRepository,
{
    repo: R,
}

impl<R> PgService<R>
where
    R: ConnectionStatsRepository
        + TableStatsRepository
        + QueryStatsRepository
        + TableSizeRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_connections(&self) -> AppResult<Vec<PgConnection>> {
        self.repo.get_connections().await
    }

    pub async fn get_table_info(&self) -> AppResult<Vec<PgTableInfo>> {
        self.repo.get_table_info().await
    }

    pub async fn get_table_indexes(&self) -> AppResult<Vec<PgTableIndex>> {
        self.repo.get_table_indexes().await
    }

    pub async fn get_stat_statements(&self) -> AppResult<Vec<PgStatStatements>> {
        self.repo.get_stat_statements().await
    }

    pub async fn get_table_sizes(&self) -> AppResult<Vec<PgStatTableSize>> {
        self.repo.get_table_sizes().await
    }

    pub async fn get_full_stats(&self) -> AppResult<PgFullStats> {
        let (connections, table_info, table_indexes, statements, sizes) = tokio::try_join!(
            self.get_connections(),
            self.get_table_info(),
            self.get_table_indexes(),
            self.get_stat_statements(),
            self.get_table_sizes(),
        )?;

        Ok(PgFullStats {
            connections,
            table_info,
            table_indexes,
            statements,
            sizes,
        })
    }
}

pub struct PgFullStats {
    pub connections: Vec<PgConnection>,
    pub table_info: Vec<PgTableInfo>,
    pub table_indexes: Vec<PgTableIndex>,
    pub statements: Vec<PgStatStatements>,
    pub sizes: Vec<PgStatTableSize>,
}
