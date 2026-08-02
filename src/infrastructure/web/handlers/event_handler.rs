use crate::application::services::EventService;
use crate::domain::repositories::EventRepository;
use crate::infrastructure::web::templates::{EventsTemplate, MsgEventTemplate, MsgSendTemplate};
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;
use std::time::Instant;
use tracing::error;

pub async fn get_events<R: EventRepository>(
    service: web::Data<EventService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_events().await {
        Ok(events) => {
            let template = EventsTemplate {
                events,
                elapsed_ms: start.elapsed().as_millis(),
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(template.render().map_err(|e| {
                    error!("Template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template render error")
                })?))
        }
        Err(e) => {
            error!("Service error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Service error"))
        }
    }
}

pub async fn get_msg_events<R: EventRepository>(
    service: web::Data<EventService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_msg_events().await {
        Ok(events) => {
            let template = MsgEventTemplate {
                msgevents: events,
                elapsed_ms: start.elapsed().as_millis(),
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(template.render().map_err(|e| {
                    error!("Template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template render error")
                })?))
        }
        Err(e) => {
            error!("Service error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Service error"))
        }
    }
}

pub async fn get_msg_sends<R: EventRepository>(
    service: web::Data<EventService<R>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match service.get_msg_sends().await {
        Ok(events) => {
            let template = MsgSendTemplate {
                msgsend: events,
                elapsed_ms: start.elapsed().as_millis(),
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(template.render().map_err(|e| {
                    error!("Template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template render error")
                })?))
        }
        Err(e) => {
            error!("Service error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Service error"))
        }
    }
}
