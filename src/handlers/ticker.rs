use crate::api::models::Ticker;
use crate::api::templates::TickersTemplate;
use crate::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn tickers(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let tickers = state.ticker_repo.get_tickers().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let tickers: Vec<(usize, Ticker)> = tickers
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            TickersTemplate {
                tickers,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
