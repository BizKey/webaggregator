use crate::api::models::EventOrder;
use crate::api::templates::EventOrderTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use tracing::error;

use sqlx::PgPool;
use std::time::Instant;

pub async fn eventorders(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let event_orders: Vec<EventOrder> = sqlx::query_as::<_, EventOrder>(
        r#"
        SELECT exchange, status, type_, symbol, side, order_type, fee_type, liquidity, price, order_id, client_oid, trade_id, origin_size, size, filled_size, match_size, match_price, canceled_size, old_size, remain_size, remain_funds, order_time, ts, updated_at
        FROM orderevent
        ORDER BY updated_at
        DESC LIMIT 1000;
        "#)
        .fetch_all(pool.as_ref())
        .await.map_err(|e|{
            error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Template render error")
        })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            EventOrderTemplate {
                event_orders,
                elapsed_ms,
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
