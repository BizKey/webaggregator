use crate::templates::IndexTemplate;
use actix_web::HttpResponse;
use askama::Template;

pub async fn index() -> HttpResponse {
    let template = IndexTemplate {};

    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::InternalServerError().body("Error template render"),
    }
}
