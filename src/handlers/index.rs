use crate::api::templates::IndexTemplate;
use actix_web::HttpResponse;
use askama::Template;

pub async fn index() -> HttpResponse {
    let template: IndexTemplate = IndexTemplate {};

    let html: String = match template.render() {
        Ok(html) => html,
        Err(_) => return HttpResponse::InternalServerError().body("Error template render"),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
