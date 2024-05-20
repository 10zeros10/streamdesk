use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    load_env_variables();
    let server_address = get_env_server_address();

    let http_server = configure_server();
    let bound_server = bind_server(server_address, &http_server)?;

    launch_server(bound_server).await
}

fn load_env_variables() {
    dotenv().ok();
}

fn get_env_server_address() -> String {
    env::var("SERVER_URL").unwrap_or_else(|_| {
        let default_server_address = "localhost:8080";
        eprintln!("SERVER_URL not found in .env file, defaulting to {}", default_server_address);
        default_server_address.into()
    })
}

fn configure_server() -> HttpServer {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(define_routes())
    })
}

fn define_routes() -> actix_web::Scope {
    web::scope("/api")
        .route("/ping", web::get().to(handle_ping_request))
}

async fn handle_ping_request() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

fn bind_server(
    server: HttpServer,
    address: &str,
) -> Result<HttpServer, std::io::Error> {
    server.bind(address).map_err(|e| {
        eprintln!("Failed to bind server to {}: {}", address, e);
        e
    })
}

async fn launch_server(server: HttpServer) -> std::io::Result<()> {
    println!("Server is running...");
    server.run().await.map_err(|e| {
        eprintln!("Error running server: {}", e);
        e
    })
}

fn main() {
    if let Err(e) = start_server().await {
        eprintln!("Server startup failed: {}", e);
    }
}