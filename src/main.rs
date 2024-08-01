use std::net::TcpListener;
use port_blog::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080")?;
    start_server(listener)?.await?;
    Ok(())
}