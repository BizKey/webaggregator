use crate::api::models::{Event, MsgEvent, MsgSend};
use crate::api::templates::{EventsTemplate, MsgEventTemplate, MsgSendTemplate};
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn events(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // events

    // time start
    let start: Instant = Instant::now();

    let events: Vec<Event> = match sqlx::query_as::<_, Event>(
        "SELECT exchange, msg, updated_at FROM events ORDER BY updated_at DESC LIMIT 1000;",
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(events) => events,
        Err(e) => {
            let msg: String = format!("Database error: {}", e);
            log::error!("{}", msg);
            return Ok(actix_web::error::ErrorInternalServerError("Database error").into());
        }
    };

    let template: EventsTemplate = EventsTemplate {
        events: events,
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

pub async fn msgevent(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // msgevent

    // time start
    let start: Instant = Instant::now();

    let msgevents =  match sqlx::query_as::<_, MsgEvent>(
        "SELECT exchange, msg, code, borrow_size, client_oid, order_id, loan_apply_id, limit_rate, reset_rate, remaining_rate, in_time, out_time, updated_at FROM msgevent ORDER BY updated_at DESC LIMIT 1000;",
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(msgevents) => msgevents,
        Err(e) => {
            let msg: String = format!("Database error: {}", e);
            log::error!("{}", msg);
            return Ok(actix_web::error::ErrorInternalServerError("Database error").into())
        }
    };

    let template: MsgEventTemplate = MsgEventTemplate {
        msgevents: msgevents,
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

pub async fn msgsend(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // msgsend

    // time start
    let start: Instant = Instant::now();

    let msgsend: Vec<MsgSend> =  match sqlx::query_as::<_, MsgSend>(
        "SELECT exchange, args_symbol, args_side, args_size, args_funds, args_price, args_time_in_force, args_type, args_auto_borrow, args_auto_repay, args_client_oid, args_order_id, updated_at FROM msgsend ORDER BY updated_at DESC LIMIT 1000;",
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(msgsend) =>msgsend,
        Err(e) => {
            let msg: String = format!("Database error: {}", e);
            log::error!("{}", msg);
            return Ok(actix_web::error::ErrorInternalServerError("Database error").into())
        }
    };

    let template: MsgSendTemplate = MsgSendTemplate {
        msgsend: msgsend,
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
