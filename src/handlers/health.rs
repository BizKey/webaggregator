use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    uptime_seconds: u64,
    version: &'static str,
}

pub async fn health(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let db_ok = state
        .pg_service
        .get_connections()
        .await
        .is_ok();

    let status = if db_ok { "healthy" } else { "unhealthy" };
    let http_status = if db_ok { 200 } else { 503 };

    Ok(HttpResponse::build(actix_web::http::StatusCode::from_u16(http_status).unwrap())
        .json(HealthResponse {
            status,
            uptime_seconds: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            version: env!("CARGO_PKG_VERSION"),
        }))
}