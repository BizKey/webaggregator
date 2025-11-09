use crate::models::Lend;
use crate::templates::{LendTemplate, LendsTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn lends(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all lend

    // time start
    let start = Instant::now();

    let all_lend = sqlx::query_as::<_, Lend>(
        "SELECT DISTINCT ON (currency) 
                exchange, currency, purchase_enable, redeem_enable, increment, 
                min_purchase_size, max_purchase_size, interest_increment, 
                min_interest_rate, market_interest_rate, max_interest_rate, 
                auto_purchase_enable 
            FROM lend",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let lend_with_index: Vec<(usize, Lend)> = all_lend
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = LendsTemplate {
        lends: lend_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
pub async fn lend(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all lend

    // time start
    let start = Instant::now();
    let currency_name = path.into_inner();

    let all_lend = sqlx::query_as::<_, Lend>(
        "SELECT 
                exchange, currency, purchase_enable, redeem_enable, increment, 
                min_purchase_size, max_purchase_size, interest_increment, 
                min_interest_rate, market_interest_rate, max_interest_rate, 
                auto_purchase_enable 
            FROM lend 
            WHERE currency = $1",
    )
    .bind(&currency_name)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let lend_with_index: Vec<(usize, Lend)> = all_lend
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = LendTemplate {
        lends: lend_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
