use crate::api::models::{Event, MsgEvent, MsgSend};
use crate::api::templates::{EventsTemplate, MsgEventTemplate, MsgSendTemplate};
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use tracing::error;

use sqlx::PgPool;
use std::time::Instant;

pub async fn events(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let events: Vec<Event> = sqlx::query_as::<_, Event>(
        r#"
        SELECT exchange, msg, updated_at
        FROM events
        ORDER BY updated_at
        DESC LIMIT 1000;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            EventsTemplate { events, elapsed_ms }
                .render()
                .map_err(|e| {
                    error!("Template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template render error")
                })?,
        ))
}

pub async fn msgevent(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let msgevents: Vec<MsgEvent> = sqlx::query_as::<_, MsgEvent>(
        r#"
        SELECT exchange, msg, code, borrow_size, client_oid, order_id, loan_apply_id, limit_rate, reset_rate, remaining_rate, in_time, out_time, updated_at
        FROM msgevent
        ORDER BY updated_at
        DESC LIMIT 1000;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e|{
        error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            MsgEventTemplate {
                msgevents,
                elapsed_ms,
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}

pub async fn msgsend(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let msgsend: Vec<MsgSend> =  sqlx::query_as::<_, MsgSend>(
        r#"
        SELECT exchange, args_symbol, args_side, args_size, args_funds, args_price, args_time_in_force, args_type, args_auto_borrow, args_auto_repay, args_client_oid, args_order_id, updated_at
        FROM msgsend
        ORDER BY updated_at
        DESC LIMIT 1000;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e|{
        error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            MsgSendTemplate {
                msgsend,
                elapsed_ms,
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
