use crate::api::templates::SendOrderTemplate;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn sendorders(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let sendorders = state
        .sendorder_service
        .get_sendorders()
        .await
        .map_err(|e| {
            error!("Service error: {}", e);
            actix_web::error::ErrorInternalServerError("Service error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            SendOrderTemplate {
                sendorders,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
