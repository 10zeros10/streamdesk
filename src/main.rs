use actix_web::{web, App, HttpServer, middleware};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URL not found in .env file");

    println!("Starting server at: {}", &server_url);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
            )
    })
    .bind(server_url)?
    .run()
    .await
}