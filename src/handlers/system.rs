use actix_web::{HttpResponse, Result};

pub async fn serve_css() -> Result<HttpResponse, std::io::Error> {
    let content = std::fs::read_to_string("./static/style.css")?;

    Ok(HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(content))
}

pub async fn favicon() -> Result<HttpResponse, std::io::Error> {
    let bytes = std::fs::read("./static/favicon.png")?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(bytes))
}
