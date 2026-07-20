use crate::api::templates::IndexTemplate;
use actix_web::{HttpResponse, Result as ActixResult};
use askama::Template;

pub async fn index() -> ActixResult<HttpResponse> {
    let html: String = IndexTemplate {}.render().map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
