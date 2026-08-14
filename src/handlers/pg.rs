use crate::api::templates::PgTemplate;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn pg(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    // Используем полную статистику из сервиса
    let stats = state.pg_service.get_full_stats().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            PgTemplate {
                pg_stats_connections: stats.connections,
                pg_stats_table_info: stats.table_info,
                pg_stats_table_index: stats.table_indexes,
                pg_stat_statements: stats.statements,
                pg_stat_table_size: stats.sizes,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
