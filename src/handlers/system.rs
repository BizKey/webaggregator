use actix_web::{HttpResponse, Result};

pub async fn serve_css() -> Result<HttpResponse, std::io::Error> {
    match std::fs::read_to_string("./static/style.css") {
        Ok(content) => Ok(HttpResponse::Ok()
            .content_type("text/css; charset=utf-8")
            .insert_header(("Cache-Control", "public, max-age=3600"))
            .body(content)),
        Err(e) => Err(e),
    }
}

pub async fn favicon() -> Result<HttpResponse, std::io::Error> {
    match std::fs::read("./static/favicon.png") {
        Ok(bytes) => Ok(HttpResponse::Ok()
            .content_type("image/png")
            .insert_header(("Cache-Control", "public, max-age=3600"))
            .body(bytes)),
        Err(e) => Err(e),
    }
}
