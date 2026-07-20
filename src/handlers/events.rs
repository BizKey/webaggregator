use crate::api::models::{Event, MsgEvent, MsgSend};
use crate::api::templates::{EventsTemplate, MsgEventTemplate, MsgSendTemplate};
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn events(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
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

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let template: EventsTemplate = EventsTemplate {
        events: events,
        elapsed_ms,
    };

    let html: String = template.render().map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn msgevent(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
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

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let template: MsgEventTemplate = MsgEventTemplate {
        msgevents: msgevents,
        elapsed_ms,
    };

    let html: String = template.render().map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn msgsend(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
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

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let template: MsgSendTemplate = MsgSendTemplate {
        msgsend: msgsend,
        elapsed_ms,
    };

    let html: String = template.render().map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
