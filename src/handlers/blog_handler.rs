use actix_web::{get, web, HttpResponse, Responder};
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::{fs, io::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    description: String,
    file_name: String,
    posted_at: String,
}

fn find_posts() -> Result<Vec<Post>, std::io::Error> {
    let mut t = ignore::types::TypesBuilder::new();
    t.add_defaults();

    let toml = match t.select("toml").build() {
        Ok(t) => t,
        Err(e) => {
            println!("{:?}", e);

            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Error building types",
            ));
        }
    };

    let walker = WalkBuilder::new("./posts").types(toml).build();

    let mut posts: Vec<Post> = Vec::new();

    for entry in walker {
        match entry {
            Ok(e) => {
                let path = e.path();

                if path.is_file() {
                    let post_content = fs::read_to_string(path)?;

                    let post: Post = toml::from_str(&post_content)
                        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
                    posts.push(post);
                }
            }
            Err(e) => {
                println!("{:?}", e);

                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Error walking directory",
                ));
            }
        }
    }

    Ok(posts)
}

#[get("/blog")]
pub async fn blog(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let mut posts = match find_posts() {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);

            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>");
        }
    };

    posts.sort_by(|a, b| b.posted_at.cmp(&a.posted_at));

    context.insert("posts", &posts);

    match templates.render("blog.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(s) => {
            println!("{:?}", s);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!")
        }
    }
}
