use crate::models::Bots;
use crate::templates::BotsTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;
pub async fn bots(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // bots

    // time start
    let start = Instant::now();

    let bots_list = sqlx::query_as::<_, Bots>(
        "SELECT exchange, entry_id, exit_tp_id, exit_sl_id, balance, updated_at FROM bots ORDER BY updated_at DESC;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let bots_with_index: Vec<(usize, Bots)> = bots_list
        .into_iter()
        .enumerate()
        .map(|(i, bot)| (i + 1, bot))
        .collect();

    let template = BotsTemplate {
        bots: bots_with_index,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
