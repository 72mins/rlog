use actix_web::{get, web, HttpResponse, Responder};
use crate::utils::extract_content::extract_content;
use crate::utils::extract_meta::extract_meta;
use crate::utils::get_html::get_html;

#[get("/contact")]
pub async fn contact(templates: web::Data<tera::Tera>) -> impl Responder {
    let content = match extract_content("templates/contact/content.md") {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", e);

            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>");
        }
    };
    
    let meta = match extract_meta("templates/contact/meta.toml") {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);

            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>");
        }
    };

    let html_output = get_html(&content);

    let mut context = tera::Context::new();
    context.insert("content", &html_output);
    context.insert("meta", &meta);

    match templates.render("contact/contact.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
