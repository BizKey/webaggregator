use crate::api::models::Currency;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait CurrencyRepository: Send + Sync {
    async fn get_currencies(&self) -> RepositoryResult<Vec<Currency>>;
}

pub struct PostgresCurrencyRepository {
    pool: PgPool,
}

impl PostgresCurrencyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CurrencyRepository for PostgresCurrencyRepository {
    async fn get_currencies(&self) -> RepositoryResult<Vec<Currency>> {
        let currencies = sqlx::query_as::<_, Currency>(
            r#"
            SELECT exchange, currency, currency_name, full_name, precision, 
                   is_margin_enabled, is_debit_enabled, updated_at
            FROM currency
            ORDER BY updated_at DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(currencies)
    }
}
