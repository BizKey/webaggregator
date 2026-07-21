use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use sqlx::PgPool;
use std::time::Instant;
use tracing::error;

use crate::api::models::Balance;
use crate::api::templates::BalanceTemplate;

pub async fn balances(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let balances: Vec<Balance> = sqlx::query_as::<_, Balance>(
        r#"
        SELECT exchange, account_id, available, available_change, currency, hold_value, hold_change, relation_event, relation_event_id, event_time, total, symbol, order_id, trade_id, updated_at
        FROM balance
        ORDER BY updated_at DESC
        LIMIT 1000;
        "#)
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| {
            error!("Database error in balances handler: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            BalanceTemplate {
                balances,
                elapsed_ms,
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
