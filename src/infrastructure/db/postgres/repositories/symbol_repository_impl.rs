use crate::domain::entities::Symbol;
use crate::domain::repositories::{RepositoryError, SymbolRepository};
use crate::infrastructure::db::postgres::models::SymbolModel;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
pub struct PostgresSymbolRepository {
    pool: PgPool,
}

impl PostgresSymbolRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SymbolRepository for PostgresSymbolRepository {
    async fn find_all(&self) -> Result<Vec<Symbol>, RepositoryError> {
        let models = sqlx::query_as::<_, SymbolModel>(
            r#"
            SELECT exchange, symbol, symbol_name, base_currency, quote_currency, 
                   fee_currency, market, base_min_size, quote_min_size, base_max_size, 
                   quote_max_size, base_increment, quote_increment, price_increment, 
                   price_limit_rate, min_funds, is_margin_enabled, enable_trading, 
                   fee_category, maker_fee_coefficient, taker_fee_coefficient, st, updated_at
            FROM symbol
            ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(Symbol::from).collect())
    }
}
