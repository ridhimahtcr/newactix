use crate::controller::posts::PaginationQuery;
use crate::model::database;
use actix_web::{web, HttpResponse};
use liquid::model::Value;
use liquid::object;
use std::fs;

pub async fn pagination_homepage(
    page_number: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let page_number = *page_number;

    let posts = database::get_posts().await?;

    let limit = 3;

    let total_pages = (posts.len() as f64 / limit as f64).ceil() as i32;

    let offset = (page_number - 1) * limit;

    let posts_array = posts
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|post| {
            let mut post_map = object!({
                "post_id": Value::scalar(post.post_id.to_string()),
                "title": Value::scalar(post.title),
                "description": Value::scalar(post.description),
            });
            Value::Object(post_map)
        })
        .collect::<Vec<Value>>();

    let pagination = object!({
        "prev": if page_number > 1 { Value::scalar(page_number - 1) } else { Value::Nil },
        "next": if page_number < total_pages { Value::scalar(page_number + 1) } else { Value::Nil },
        "current": Value::scalar(page_number),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let mut context = object!({
        "posts": Value::Array(posts_array),
        "pagination": Value::Object(pagination),
    });

    let html_template =
        fs::read_to_string("templates/admin.html").expect("Failed to read the file");

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

pub async fn homepage(
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = database::get_posts().await?;

    let limit = 3;

    let PaginationQuery { page_number } = query.into_inner();

    let page = page_number.unwrap_or(1);

    let total_pages = (posts.len() as f64 / limit as f64).ceil() as i32;

    let offset = (page - 1) * limit;

    let posts_array = posts
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|post| {
            let mut post_map = object!({
                "post_id": Value::scalar(post.post_id.to_string()),
             "title" : Value::scalar(post.title),
            "description" : Value::scalar(post.description),
            });
            Value::Object(post_map)
        })
        .collect::<Vec<Value>>();

    let html_template =
        fs::read_to_string("templates/admin.html").expect("Failed to read the file");

    let pagination = object!({
        "prev": if page > 1 { Value::scalar(page - 1) } else { Value::Nil },
        "next": if page < total_pages { Value::scalar(page + 1) } else { Value::Nil },
        "current": Value::scalar(page),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let mut context = object!({
        "posts":  Value::Array(posts_array),
         "pagination": Value::Object(pagination),
    });

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
