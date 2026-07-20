use crate::api::models::Bots;
use crate::api::templates::BotsTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn bots(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let bots_list: Vec<Bots> = sqlx::query_as::<_, Bots>(
        r#"
        SELECT exchange, entry_client_oid, exit_tp_order_id, exit_tp_client_oid, exit_sl_order_id, exit_sl_client_oid, symbol, balance, updated_at
        FROM bots
        ORDER BY updated_at DESC;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let bots: Vec<(usize, Bots)> = bots_list
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

    let final_balance: f64 = bots
        .iter()
        .filter_map(|(_, bot)| bot.balance.as_ref().and_then(|s| s.parse::<f64>().ok()))
        .sum();

    let bots_count = bots.len();
    let init_balance: f64 = (20 * bots_count) as f64;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = BotsTemplate {
        bots,
        init_balance,
        final_balance,
        elapsed_ms,
    }
    .render()
    .map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
