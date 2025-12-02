use crate::models::{ActiveOrder, EventOrder};
use crate::templates::{ActiveOrderTemplate, EventOrderTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;
pub async fn activeorders(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // activeorders

    // time start
    let start = Instant::now();

    let active_orders = sqlx::query_as::<_, ActiveOrder>(
        "SELECT exchange, order_id, symbol, side FROM orderactive;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let template = ActiveOrderTemplate {
        active_orders: active_orders,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
pub async fn eventorders(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // eventorders

    // time start
    let start = Instant::now();

    let event_orders =
        sqlx::query_as::<_, EventOrder>("SELECT exchange, order_id, symbol, side FROM orderevent;")
            .fetch_all(pool.get_ref())
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?;

    let template = EventOrderTemplate {
        event_orders: event_orders,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
