use crate::api::models::Symbol;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait SymbolRepository: Send + Sync {
    async fn get_all_symbols(&self) -> RepositoryResult<Vec<Symbol>>;
    async fn get_tradeable_symbols(&self) -> RepositoryResult<Vec<Symbol>>;
}

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
    async fn get_all_symbols(&self) -> RepositoryResult<Vec<Symbol>> {
        let symbols = sqlx::query_as::<_, Symbol>(
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
        .await?;

        Ok(symbols)
    }

    async fn get_tradeable_symbols(&self) -> RepositoryResult<Vec<Symbol>> {
        let symbols = sqlx::query_as::<_, Symbol>(
            r#"
            SELECT exchange, symbol, symbol_name, base_currency, quote_currency, 
                   fee_currency, market, base_min_size, quote_min_size, base_max_size, 
                   quote_max_size, base_increment, quote_increment, price_increment, 
                   price_limit_rate, min_funds, is_margin_enabled, enable_trading, 
                   fee_category, maker_fee_coefficient, taker_fee_coefficient, st, updated_at
            FROM symbol
            WHERE is_margin_enabled = true 
              AND enable_trading = true 
              AND fee_category = 1 
              AND quote_currency = 'USDT' 
              AND base_currency <> 'USDC' 
              AND base_currency <> 'KCS' 
            ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(symbols)
    }
}
