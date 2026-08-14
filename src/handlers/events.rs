use crate::api::templates::{EventsTemplate, MsgEventTemplate, MsgSendTemplate};
use crate::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn events(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let events = state.event_repo.get_events().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            EventsTemplate {
                events,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}

pub async fn msgevent(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let msgevents = state.msgevent_repo.get_msgevents().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            MsgEventTemplate {
                msgevents,
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}

pub async fn msgsend(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    let msgsends = state.msgsend_repo.get_msgsends().await.map_err(|e| {
        error!("Repository error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            MsgSendTemplate {
                msgsend: msgsends, // Исправлено: поле называется msgsend, а не msgsends
                elapsed_ms: start.elapsed().as_millis(),
            }
            .render()
            .map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template render error")
            })?,
        ))
}
