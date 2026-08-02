use crate::application::services::BalanceService;
use crate::domain::repositories::BalanceRepository;
use crate::infrastructure::web::templates::BalanceTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_balances<R: BalanceRepository>(
    service: web::Data<BalanceService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_all_balances().await {
        Ok(balances) => {
            let template = BalanceTemplate {
                balances,
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
