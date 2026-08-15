use crate::api::models::Currency;
use crate::api::templates::CurrenciesTemplate;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn currencies(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let currencies = state.currency_service.get_currencies().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    let currencies: Vec<(usize, Currency)> = currencies
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            CurrenciesTemplate {
                currencies,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
