use crate::application::services::BotService;
use crate::domain::repositories::BotRepository;
use crate::infrastructure::web::templates::BotsTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_bots<R: BotRepository>(
    service: web::Data<BotService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_all_bots().await {
        Ok(bots) => {
            let bots_with_index: Vec<(usize, _)> = bots
                .into_iter()
                .enumerate()
                .map(|(i, v)| (i + 1, v))
                .collect();

            let final_balance = bots_with_index
                .iter()
                .filter_map(|(_, bot)| bot.balance.as_ref().and_then(|s| s.parse::<f64>().ok()))
                .sum();

            let bots_count = bots_with_index.len();
            let init_balance = (20 * bots_count) as f64;

            let template = BotsTemplate {
                bots: bots_with_index,
                init_balance,
                final_balance,
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
