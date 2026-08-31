use crate::api::models::EventOrder;
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, FromRow)]
pub struct TradeWithStop {
    // Поля из eventorder
    pub order_id: String,
    pub client_oid: Option<String>,
    pub symbol: String,
    pub side: String,
    pub price: Option<String>,
    pub size: Option<String>,
    pub filled_size: Option<String>,
    pub status: String,
    pub event_updated_at: chrono::DateTime<chrono::Utc>,
    pub stop_type: Option<String>,
    pub stop_price: Option<String>,
    pub stop_size: Option<String>,
    pub stop_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait]
pub trait EventOrderRepository: Send + Sync {
    async fn get_event_orders(&self) -> RepositoryResult<Vec<EventOrder>>;
    async fn get_trades_with_stops(
        &self,
        symbol: &str,
        limit: i64,
    ) -> RepositoryResult<Vec<TradeWithStop>>;
}

pub struct PostgresEventOrderRepository {
    pool: PgPool,
}

impl PostgresEventOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventOrderRepository for PostgresEventOrderRepository {
    async fn get_event_orders(&self) -> RepositoryResult<Vec<EventOrder>> {
        let event_orders = sqlx::query_as::<_, EventOrder>(
            r#"
            SELECT exchange, status, type_, symbol, side, order_type, fee_type, 
                   liquidity, price, order_id, client_oid, trade_id, origin_size, 
                   size, filled_size, match_size, match_price, canceled_size, 
                   old_size, remain_size, remain_funds, order_time, ts, updated_at
            FROM orderevent
            ORDER BY updated_at DESC LIMIT 1000;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(event_orders)
    }
    async fn get_trades_with_stops(
        &self,
        symbol: &str,
        limit: i64,
    ) -> RepositoryResult<Vec<TradeWithStop>> {
        let trades = sqlx::query_as::<_, TradeWithStop>(
            r#"
            SELECT DISTINCT ON (s.client_oid)
                s.symbol,
                s.stop_type,
                s.stop_price,
                s.size as stop_size,
                s.updated_at as stop_updated_at,
                e.order_id,
                e.client_oid,
                e.side,
                e.price,
                e.size,
                e.filled_size,
                e.status,
                e.updated_at as event_updated_at
            FROM stoporders s
            INNER JOIN orderevent e ON 
                e.client_oid IS NOT NULL 
                AND e.client_oid = s.client_oid
            WHERE s.symbol = $1
            ORDER BY s.client_oid, e.updated_at DESC
            LIMIT $2;
            "#,
        )
        .bind(symbol)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(trades)
    }
}
