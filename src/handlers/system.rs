use actix_web::{HttpResponse, Result as ActixResult};

pub async fn serve_css() -> ActixResult<HttpResponse, std::io::Error> {
    let content: String = match std::fs::read_to_string("./static/style.css") {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(content))
}

pub async fn favicon() -> ActixResult<HttpResponse, std::io::Error> {
    let content: Vec<u8> = match std::fs::read("./static/favicon.png") {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(content))
}
