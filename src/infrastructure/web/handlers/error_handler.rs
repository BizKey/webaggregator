use crate::application::services::ErrorService;
use crate::domain::repositories::ErrorRepository;
use crate::infrastructure::web::templates::ErrorsTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_errors<R: ErrorRepository>(
    service: web::Data<ErrorService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_all_errors().await {
        Ok(errors) => {
            let template = ErrorsTemplate {
                errors,
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
