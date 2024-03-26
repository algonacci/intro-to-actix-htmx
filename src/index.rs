use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    message: String,
}

pub async fn index() -> impl Responder {
    let template = IndexTemplate {
        title: "Data KRL".to_string(),
        message: "Hello, world!".to_string(),
    };
    let rendered_html = template.render().unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered_html)
}
