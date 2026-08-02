use crate::application::services::TickerService;
use crate::domain::repositories::TickerRepository;
use crate::infrastructure::web::templates::TickersTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_tickers<R: TickerRepository>(
    service: web::Data<TickerService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_all_tickers().await {
        Ok(tickers) => {
            let tickers_with_index: Vec<(usize, _)> = tickers
                .into_iter()
                .enumerate()
                .map(|(i, v)| (i + 1, v))
                .collect();

            let template = TickersTemplate {
                tickers: tickers_with_index,
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
