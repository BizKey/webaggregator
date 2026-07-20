use crate::api::models::{PositionAsset, PositionDebt, PositionRatio};
use crate::api::templates::{PositinRatioTemplate, PositionAssetTemplate, PositionDebtTemplate};
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use log;
use sqlx::PgPool;
use std::time::Instant;

pub async fn positionasset(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let position_asset: Vec<PositionAsset> = sqlx::query_as::<_, PositionAsset>(
        "SELECT exchange, asset_symbol, asset_total, asset_available, asset_hold, updated_at FROM positionasset ORDER BY updated_at DESC LIMIT 1000;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = PositionAssetTemplate {
        position_asset: position_asset,
        elapsed_ms,
    }
    .render()
    .map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
pub async fn positiondebt(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let position_debt: Vec<PositionDebt> = sqlx::query_as::<_, PositionDebt>(
        "SELECT exchange, debt_symbol, debt_value, updated_at FROM positiondebt ORDER BY updated_at DESC LIMIT 1000;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = PositionDebtTemplate {
        position_debt: position_debt,
        elapsed_ms,
    }
    .render()
    .map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
pub async fn positionratio(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let position_ratio = sqlx::query_as::<_, PositionRatio>(
        "SELECT exchange, debt_ratio, total_asset, margin_coefficient_total_asset, total_debt, updated_at FROM positionratio ORDER BY updated_at DESC LIMIT 1000;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = PositinRatioTemplate {
        position_ratio: position_ratio,
        elapsed_ms,
    }
    .render()
    .map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
