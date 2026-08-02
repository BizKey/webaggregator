use crate::application::services::PositionService;
use crate::domain::repositories::PositionRepository;
use crate::infrastructure::web::templates::{
    PositionAssetTemplate, PositionDebtTemplate, PositionRatioTemplate,
};
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_position_assets<R: PositionRepository>(
    service: web::Data<PositionService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_assets().await {
        Ok(assets) => {
            let template = PositionAssetTemplate {
                position_asset: assets,
                elapsed_ms: start.elapsed().as_millis(),
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(template.render().map_err(|e| {
                    error!("Template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template render error")
                })?))
        }
        Err(e) => {
            error!("Service error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Service error"))
        }
    }
}

pub async fn get_position_debts<R: PositionRepository>(
    service: web::Data<PositionService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_debts().await {
        Ok(debts) => {
            let template = PositionDebtTemplate {
                position_debt: debts,
                elapsed_ms: start.elapsed().as_millis(),
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(template.render().map_err(|e| {
                    error!("Template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template render error")
                })?))
        }
        Err(e) => {
            error!("Service error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Service error"))
        }
    }
}

pub async fn get_position_ratios<R: PositionRepository>(
    service: web::Data<PositionService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_ratios().await {
        Ok(ratios) => {
            let template = PositionRatioTemplate {
                position_ratio: ratios,
                elapsed_ms: start.elapsed().as_millis(),
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(template.render().map_err(|e| {
                    error!("Template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template render error")
                })?))
        }
        Err(e) => {
            error!("Service error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Service error"))
        }
    }
}
