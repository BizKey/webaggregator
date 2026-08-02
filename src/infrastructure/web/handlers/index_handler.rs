use crate::infrastructure::web::templates::IndexTemplate;
use actix_web::{HttpResponse, Result as ActixResult};
use askama::Template;
use tracing::error;

pub async fn get_index() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(IndexTemplate {}.render().map_err(|e| {
            error!("Template render error: {}", e);
            actix_web::error::ErrorInternalServerError("Template render error")
        })?))
}
