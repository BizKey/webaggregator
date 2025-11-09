use crate::models::Borrow;
use crate::templates::{BorrowTemplate, BorrowsTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn borrows(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all borrow

    // time start
    let start = Instant::now();

    let all_borrow = sqlx::query_as::<_, Borrow>(
        "SELECT DISTINCT ON (currency) 
                exchange, currency, hourly_borrow_rate, annualized_borrow_rate 
            FROM borrow",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let borrow_with_index: Vec<(usize, Borrow)> = all_borrow
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = BorrowsTemplate {
        borrows: borrow_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn borrow(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // all borrow

    // time start
    let start = Instant::now();
    let currency_name = path.into_inner();

    let all_borrow = sqlx::query_as::<_, Borrow>(
        "SELECT 
                exchange, currency, hourly_borrow_rate, annualized_borrow_rate 
            FROM borrow 
            WHERE currency = $1",
    )
    .bind(&currency_name)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let borrow_with_index: Vec<(usize, Borrow)> = all_borrow
        .into_iter()
        .enumerate()
        .map(|(i, currency)| (i + 1, currency))
        .collect();

    let template = BorrowTemplate {
        borrows: borrow_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
