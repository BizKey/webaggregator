use crate::application::services::SymbolService;
use crate::domain::repositories::SymbolRepository;
use crate::infrastructure::web::templates::SymbolsTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_symbols<R: SymbolRepository>(
    service: web::Data<SymbolService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_all_symbols().await {
        Ok(symbols) => {
            let symbols_with_index: Vec<(usize, _)> = symbols
                .into_iter()
                .enumerate()
                .map(|(i, v)| (i + 1, v))
                .collect();

            let template = SymbolsTemplate {
                symbols: symbols_with_index,
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
