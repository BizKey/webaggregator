use crate::api::templates::PgTemplate;
use crate::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn pg(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let pg_stats_connections = state.pg_repo.get_connections().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stats_table_info = state.pg_repo.get_table_info().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stats_table_index = state.pg_repo.get_table_indexes().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stat_statements = state.pg_repo.get_stat_statements().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stat_table_size = state.pg_repo.get_table_sizes().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            PgTemplate {
                pg_stats_connections,
                pg_stats_table_info,
                pg_stats_table_index,
                pg_stat_statements,
                pg_stat_table_size,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
