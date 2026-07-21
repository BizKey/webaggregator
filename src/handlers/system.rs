use actix_web::{HttpResponse, Result as ActixResult};
use tracing::error;

pub async fn serve_css() -> ActixResult<HttpResponse> {
    let content = std::fs::read_to_string("./static/style.css").map_err(|e| {
        error!("Fail read ./static/style.css: {}", e);
        actix_web::error::ErrorInternalServerError("Fail")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(content))
}

pub async fn favicon() -> ActixResult<HttpResponse> {
    let content: Vec<u8> = std::fs::read("./static/favicon.png").map_err(|e| {
        error!("Fail read ./static/favicon.png: {}", e);
        actix_web::error::ErrorInternalServerError("Fail")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(content))
}
