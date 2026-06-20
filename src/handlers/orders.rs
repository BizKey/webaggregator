use crate::api::models::EventOrder;
use crate::api::templates::EventOrderTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn eventorders(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // eventorders

    // time start
    let start: Instant = Instant::now();

    let event_orders: Vec<EventOrder> = match sqlx::query_as::<_, EventOrder>("SELECT exchange, status, type_, symbol, side, order_type, fee_type, liquidity, price, order_id, client_oid, trade_id, origin_size, size, filled_size, match_size, match_price, canceled_size, old_size, remain_size, remain_funds, order_time, ts, updated_at FROM orderevent ORDER BY updated_at DESC LIMIT 1000;")
            .fetch_all(pool.get_ref())
            .await {
                Ok(event_orders) => event_orders,
                Err(e) => {
                    eprintln!("Database error: {}", e);
                  return  Ok(actix_web::error::ErrorInternalServerError("Database error").into())
                }
            };

    let template: EventOrderTemplate = EventOrderTemplate {
        event_orders: event_orders,
        elapsed_ms: start.elapsed().as_millis(),
    };

    let html: String = match template.render() {
        Ok(html) => html,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Error template render")),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
