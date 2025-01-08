pub mod handlers;
pub mod models;
pub mod enums;
pub mod services;
pub mod configuration;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[+] Starting web application in port 8000.");
    HttpServer::new(|| {

        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();


        let app = App::new()
            .wrap_fn(|req, srv|{
                println!("{}-{}", req.method(), req.uri());
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(handlers::views_factory).wrap(cors);
        return app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}