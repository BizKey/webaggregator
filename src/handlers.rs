use actix_web::HttpResponse;
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
