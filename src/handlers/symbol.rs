use crate::api::templates::SymbolsTemplate;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn symbols(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let symbols = state
        .symbol_service
        .get_symbols_with_index(false)
        .await
        .map_err(|e| {
            error!("Service error: {}", e);
            actix_web::error::ErrorInternalServerError("Service error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            SymbolsTemplate {
                symbols,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}

pub async fn tradeable(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let symbols = state
        .symbol_service
        .get_symbols_with_index(true)
        .await
        .map_err(|e| {
            error!("Service error: {}", e);
            actix_web::error::ErrorInternalServerError("Service error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            SymbolsTemplate {
                symbols,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
