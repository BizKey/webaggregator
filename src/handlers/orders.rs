use crate::api::templates::EventOrderTemplate;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn eventorders(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let event_orders = state.order_service.get_event_orders().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            EventOrderTemplate {
                event_orders,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
