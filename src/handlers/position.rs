use crate::api::templates::{PositinRatioTemplate, PositionAssetTemplate, PositionDebtTemplate};
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn positionasset(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let position_asset = state
        .position_service
        .get_position_assets()
        .await
        .map_err(|e| {
            error!("Service error: {}", e);
            actix_web::error::ErrorInternalServerError("Service error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            PositionAssetTemplate {
                position_asset,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}

pub async fn positiondebt(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let position_debt = state
        .position_service
        .get_position_debts()
        .await
        .map_err(|e| {
            error!("Service error: {}", e);
            actix_web::error::ErrorInternalServerError("Service error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            PositionDebtTemplate {
                position_debt,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}

pub async fn positionratio(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let position_ratio = state
        .position_service
        .get_position_ratios()
        .await
        .map_err(|e| {
            error!("Service error: {}", e);
            actix_web::error::ErrorInternalServerError("Service error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            PositinRatioTemplate {
                position_ratio,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
