use crate::handlers::blog_handler::Post;
use actix_web::{get, web, HttpResponse, Responder};
use pulldown_cmark::{Options, Parser};
use std::{fs, io::Error};

fn extract_markdown(post_name: &str) -> Result<String, Error> {
    let markdown = match fs::read_to_string(format!("./posts/{}/post.md", post_name)) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);

            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Error reading markdown file",
            ));
        }
    };

    Ok(markdown)
}

fn extract_post(post_name: &str) -> Result<Post, Error> {
    let post_content = match fs::read_to_string(format!("./posts/{}/post.toml", post_name)) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);

            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Error reading post file",
            ));
        }
    };

    let post: Post = match toml::from_str(&post_content) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);

            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Error parsing post file",
            ));
        }
    };

    Ok(post)
}

#[get("/blog/{post_name}")]
pub async fn blog_post(
    template: web::Data<tera::Tera>,
    post_name: web::Path<String>,
) -> impl Responder {
    let mut context = tera::Context::new();
    let options = Options::empty();

    let markdown = match extract_markdown(&post_name) {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);
            
            return match template.render("not_found.html", &context) {
                Ok(s) => HttpResponse::NotFound().content_type("text/html").body(s),
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError()
                        .content_type("text/html")
                        .body("<p>Something went wrong!</p>")
                }
            };
        }
    };

    let post = match extract_post(&post_name) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);

            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>");
        }
    };

    let parser = Parser::new_ext(&markdown, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta", &post);

    match template.render("post.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
