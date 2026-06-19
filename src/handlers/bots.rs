use crate::api::models::Bots;
use crate::api::templates::BotsTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;
pub async fn bots(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // bots

    // time start
    let start: Instant = Instant::now();

    let bots_list: Vec<Bots> =  match sqlx::query_as::<_, Bots>(
        "
        SELECT exchange, entry_client_oid, exit_tp_order_id, exit_tp_client_oid, exit_sl_order_id, exit_sl_client_oid, symbol, balance, updated_at
        FROM bots
        ORDER BY updated_at DESC;
        ",
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(bots_list) => bots_list,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return  Ok(actix_web::error::ErrorInternalServerError("Database error").into())
        }
    };

    let bots_with_index: Vec<(usize, Bots)> = bots_list
        .into_iter()
        .enumerate()
        .map(|(i, bot)| (i + 1, bot))
        .collect();

    let final_balance: f64 = bots_with_index
        .iter()
        .filter_map(|(_, bot)| bot.balance.as_ref().and_then(|s| s.parse::<f64>().ok()))
        .sum();

    let bots_count = bots_with_index.len();
    let init_balance_value = (20 * bots_count) as f64;

    let template: BotsTemplate = BotsTemplate {
        bots: bots_with_index,
        init_balance: init_balance_value,
        final_balance: final_balance,
        elapsed_ms: start.elapsed().as_millis(),
    };

    let html: String = match template.render() {
        Ok(html) => html,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Error template render")),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
