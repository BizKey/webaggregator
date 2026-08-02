use crate::application::services::PgStatService;
use crate::domain::repositories::PgStatRepository;
use crate::infrastructure::web::templates::PgTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_pg_stats<R: PgStatRepository>(
    service: web::Data<PgStatService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let connections = service.get_connections().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    let table_info = service.get_table_info().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    let table_index = service.get_table_index().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    let statements = service.get_statements().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    let table_sizes = service.get_table_sizes().await.map_err(|e| {
        error!("Service error: {}", e);
        actix_web::error::ErrorInternalServerError("Service error")
    })?;

    let template = PgTemplate {
        pg_stats_connections: connections,
        pg_stats_table_info: table_info,
        pg_stats_table_index: table_index,
        pg_stat_statements: statements,
        pg_stat_table_size: table_sizes,
        elapsed_ms: start.elapsed().as_millis(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().map_err(|e| {
            error!("Template render error: {}", e);
            actix_web::error::ErrorInternalServerError("Template render error")
        })?))
}
