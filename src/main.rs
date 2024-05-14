use actix_web::{web, App, HttpServer, middleware};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn start_http_server() -> std::io::Result<()> {
    dotenv().ok();
    let http_server_address = env::var("SERVER_URL").expect("SERVER_URL not found in .env file");

    println!("Starting HTTP server at: {}", &http_server_address);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                // Here, you can add your API endpoints and their respective handlers
            )
    })
    .bind(http_server_address)?
    .run()
    .await
}