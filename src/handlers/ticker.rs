use crate::api::models::Ticker;
use crate::api::templates::TickersTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn tickers(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    // all tickers

    // time start
    let start: Instant = Instant::now();

    let tickers: Vec<Ticker> = match sqlx::query_as::<_, Ticker>(
        "
        SELECT exchange, symbol, symbol_name, taker_fee_rate, maker_fee_rate, taker_coefficient, maker_coefficient, updated_at
        FROM ticker
        ORDER BY updated_at DESC;
        ",
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(tickers) => tickers,
        Err(e) => {
            let msg: String = format!("Database error: {}", e);
            log::error!("{}", msg);
            return Ok(actix_web::error::ErrorInternalServerError("Database error").into())
        }
    };

    let tickers_with_index: Vec<(usize, Ticker)> = tickers
        .into_iter()
        .enumerate()
        .map(|(i, ticker)| (i + 1, ticker))
        .collect();

    let template: TickersTemplate = TickersTemplate {
        tickers: tickers_with_index,
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
