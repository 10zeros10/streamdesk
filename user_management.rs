use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Deserialize)]
struct LoginDetails {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthTokenResponse {
    token: String,
}

#[derive(Deserialize)]
struct ProfileUpdateRequest {
    bio: Option<String>,
}

#[derive(Serialize)]
struct ProfileResponse {
    id: i32,
    username: String,
    bio: Option<String>,
}

async fn authenticate_user(credentials: web::Json<LoginDetails>, db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let user_query_result = sqlx::query!(
        "SELECT * FROM users WHERE username = $1 AND password = $2",
        credentials.username,
        credentials.password
    )
    .fetch_optional(&**db_pool)
    .await;

    match user_query_result {
        Ok(Some(_user)) => {
            HttpResponse::Ok().json(AuthTokenResponse { token: "fake_token".to_string() }) 
        },
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        },
    }
}

async fn update_profile(user_id: web::Path<i32>, profile_data: web::Json<ProfileUpdateRequest>, db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match sqlx::query!(
        "UPDATE users SET bio = $1 WHERE id = $2",
        profile_data.bio,
        user_id.0
    )
    .execute(&**db_pool)
    .await {
        Ok(_) => HttpResponse::Ok().json(ProfileResponse {
            id: user_id.0,
            username: "ExampleUser".to_string(),
            bio: profile_data.bio.clone(),
        }),
        Err(e) => {
            log::error!("Failed to execute update query: {:?}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

async fn verify_auth_token(request: web::HttpRequest) -> Result<(), actix_web::Error> {
    let auth_header = request.headers().get("Authorization").and_then(|v| v.to_str().ok());

    match auth_header {
        Some(header_value) if header_value == "Bearer fake_token" => Ok(()),
        _ => Err(actix_web::error::ErrorUnauthorized("Invalid Token")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_connection_string)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/login")
                    .route(web::post().to(authenticate_user))
            )
            .service(
                web::resource("/user/{id}/profile")
                    .route(web::put().to(update_profile))
                    .wrap_fn(|req, srv| {
                        verify_auth_token(req).and_then(move |_| srv.call(req))
                    })
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}