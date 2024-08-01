use actix_web::{web, HttpResponse, Responder};

pub async fn not_found(templates: web::Data<tera::Tera>) -> impl Responder {
    match templates.render("not_found.html", &tera::Context::new()) {
        Ok(s) => HttpResponse::NotFound().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}