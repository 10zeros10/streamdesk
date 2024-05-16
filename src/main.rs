use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn start_http_server() -> std::io::Result<()> {
    dotenv().ok();
    // Using .unwrap_or_else to provide a fallback value or handle the error more gracefully
    let http_server_address = env::var("SERVER_URL").unwrap_or_else(|_| {
        eprintln!("SERVER_URL not found in .env file, defaulting to localhost:8080");
        "localhost:8080".into() // Provide a default address or handle it as an error, as suited
    });

    println!("Starting HTTP server at: {}", &http_server_address);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // Example ping endpoint for basic API structure
            .service(
                web::scope("/api")
                .route("/ping", web::get().to(|| async { HttpResponse::Ok().body("pong") }))
                // Here, you can add more of your API endpoints and their respective handlers
            )
    });

    // Improved error handling for server binding
    let server = match server.bind(&http_server_address) {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", &http_server_address, e);
            return Err(e); // Exit if binding fails
        }
    };

    // Improved error handling for server run
    match server.run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Server run error: {}", e);
            Err(e) // Propagate the error
        }
    }
}

fn main() {
    if let Err(e) = start_http_server().await {
        eprintln!("Server failed to start: {}", e);
    }
}