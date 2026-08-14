use crate::api::templates::BotsTemplate;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn bots(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let stats = state.bot_service.get_bots_with_stats().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            BotsTemplate {
                bots: stats.bots,
                init_balance: stats.init_balance,
                final_balance: stats.final_balance,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
