use crate::api::templates::IndexTemplate;
use actix_web::{HttpResponse, Result as ActixResult};
use askama::Template;

pub async fn index() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(IndexTemplate {}.render().map_err(|e| {
            log::error!("Template render error: {}", e);
            actix_web::error::ErrorInternalServerError("Template render error")
        })?))
}
