use actix_web::{HttpResponse, web};
use askama::Template;
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
    time: String,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Welcome to rr")
}

pub async fn hello() -> HttpResponse {
    let template = IndexTemplate {
        name: "User".to_string(),
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::InternalServerError().body("Error template render"),
    }
}
pub async fn hellodirect(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();

    let template = IndexTemplate {
        name: name,
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::InternalServerError().body("Error template render"),
    }
}

pub async fn serve_css() -> Result<HttpResponse, std::io::Error> {
    let content = std::fs::read_to_string("static/style.css")?;

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
