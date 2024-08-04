use actix_web::{get, web, HttpResponse, Responder};

#[get("/about")]
pub async fn about(templates: web::Data<tera::Tera>) -> impl Responder {
    let context = tera::Context::new();

    match templates.render("about.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!")
        }
    }
}
