use crate::api::models::Balance;
use crate::api::templates::BalanceTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn balances(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // balances

    // time start
    let start: Instant = Instant::now();

    let balances: Vec<Balance> =  match sqlx::query_as::<_, Balance>("SELECT exchange, account_id, available, available_change, currency, hold_value, hold_change, relation_event, relation_event_id, event_time, total, symbol, order_id, trade_id, updated_at FROM balance ORDER BY updated_at DESC LIMIT 1000;")
        .fetch_all(pool.get_ref())
        .await {
            Ok(balances) => balances,
            Err(e) => {
                let msg: String = format!("Database error: {}", e);
                log::error!("{}", msg);
                return  Ok(actix_web::error::ErrorInternalServerError("Database error").into())
            }
        };

    let template: BalanceTemplate = BalanceTemplate {
        balances: balances,
        elapsed_ms: start.elapsed().as_millis(),
    };

    let html: String = match template.render() {
        Ok(html) => html,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Error template render")),
    };

    let response: HttpResponse = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);

    Ok(response)
}
