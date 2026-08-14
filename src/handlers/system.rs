use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use tracing::error;

pub async fn serve_css(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    match state.static_service.get_css().await {
        Ok(file) => Ok(HttpResponse::Ok()
            .content_type(file.content_type)
            .insert_header(("Cache-Control", "public, max-age=3600"))
            .insert_header(("ETag", file.etag))
            .body(file.content)),
        Err(e) => {
            error!("Failed to serve CSS: {}", e);
            Ok(HttpResponse::InternalServerError().body("CSS not found"))
        }
    }
}

pub async fn favicon(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    match state.static_service.get_favicon().await {
        Ok(file) => Ok(HttpResponse::Ok()
            .content_type(file.content_type)
            .insert_header(("Cache-Control", "public, max-age=86400")) // 24 часа
            .insert_header(("ETag", file.etag))
            .body(file.content)),
        Err(e) => {
            error!("Failed to serve favicon: {}", e);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
