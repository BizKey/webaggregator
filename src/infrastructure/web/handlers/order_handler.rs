use crate::application::services::OrderService;
use crate::domain::repositories::OrderRepository;
use crate::infrastructure::web::templates::EventOrderTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_orders<R: OrderRepository>(
    service: web::Data<OrderService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_all_orders().await {
        Ok(orders) => {
            let template = EventOrderTemplate {
                event_orders: orders,
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
