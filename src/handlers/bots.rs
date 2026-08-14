use crate::api::models::Bot;
use crate::api::templates::BotsTemplate;
use crate::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn bots(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let bots_list = state.bot_repo.get_bots().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let bots: Vec<(usize, Bot)> = bots_list
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

    let final_balance = bots
        .iter()
        .filter_map(|(_, bot)| bot.balance.as_ref().and_then(|s| s.parse::<f64>().ok()))
        .sum();

    let bots_count = bots.len();
    let init_balance = (20 * bots_count) as f64;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            BotsTemplate {
                bots,
                init_balance,
                final_balance,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
