use std::fs;
use actix_web::HttpResponse;

pub async fn login() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/login.html").expect("Failed to read the file");

    let context = liquid::Object::new();

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}