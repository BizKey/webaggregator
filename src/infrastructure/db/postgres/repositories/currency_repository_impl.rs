use crate::domain::entities::Currency;
use crate::domain::repositories::{CurrencyRepository, RepositoryError};
use crate::infrastructure::db::postgres::models::CurrencyModel;
use async_trait::async_trait;
use sqlx::PgPool;
#[derive(Clone)]
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
    async fn find_all(&self) -> Result<Vec<Currency>, RepositoryError> {
        let models = sqlx::query_as::<_, CurrencyModel>(
            r#"
            SELECT exchange, currency, currency_name, full_name, precision, 
                   is_margin_enabled, is_debit_enabled, updated_at
            FROM currency
            ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(models.into_iter().map(Currency::from).collect())
    }
}
