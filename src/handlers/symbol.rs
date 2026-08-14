use crate::api::models::Symbol;
use crate::api::templates::SymbolsTemplate;
use crate::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn symbols(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let symbols = state.symbol_repo.get_all_symbols().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let symbols: Vec<(usize, Symbol)> = symbols
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

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
        .symbol_repo
        .get_tradeable_symbols()
        .await
        .map_err(|e| {
            error!("Repository error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let symbols: Vec<(usize, Symbol)> = symbols
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

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
