use crate::api::models::Ticker;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait TickerRepository: Send + Sync {
    async fn get_tickers(&self) -> RepositoryResult<Vec<Ticker>>;
}

pub struct PostgresTickerRepository {
    pool: PgPool,
}

impl PostgresTickerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TickerRepository for PostgresTickerRepository {
    async fn get_tickers(&self) -> RepositoryResult<Vec<Ticker>> {
        let tickers = sqlx::query_as::<_, Ticker>(
            r#"
            SELECT exchange, symbol, symbol_name, taker_fee_rate, maker_fee_rate, 
                   taker_coefficient, maker_coefficient, updated_at
            FROM ticker
            ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tickers)
    }
}
