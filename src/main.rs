use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn start_http_server() -> std::io::Result<()> {
    load_environment();
    let http_server_address = get_server_address();

    let server = create_server();
    let server = bind_server_to_address(server, &http_server_address)?;

    run_server(server).await
}

fn load_environment() {
    dotenv().ok();
}

fn get_server_address() -> String {
    env::var("SERVER_URL").unwrap_or_else(|_| {
        let default_address = "localhost:8080";
        eprintln!("SERVER_URL not found in .env file, defaulting to {}", default_address);
        default_address.into()
    })
}

fn create_server() -> HttpServer {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(api_routes())
    })
}

fn api_routes() -> actix_web::Scope {
    web::scope("/api")
        .route("/ping", web::get().to(ping_handler))
}

async fn ping_handler() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

fn bind_server_to_address(
    server: HttpServer,
    address: &str,
) -> Result<HttpServer, std::io::Error> {
    server.bind(address).map_err(|e| {
        eprintln!("Failed to bind to {}: {}", address, e);
        e
    })
}

async fn run_server(server: HttpServer) -> std::io::Result<()> {
    println!("Server running...");
    server.run().await.map_err(|e| {
        eprintln!("Server run error: {}", e);
        e
    })
}

fn main() {
    if let Err(e) = start_http_server().await {
        eprintln!("Server failed to start: {}", e);
    }
}