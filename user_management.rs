use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
struct UserProfileRequest {
    bio: Option<String>,
}

#[derive(Serialize)]
struct UserProfileResponse {
    id: i32,
    username: String,
    bio: Option<String>,
}

async fn login(login_req: web::Json<LoginRequest>, db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let user_result = sqlx::query!(
        "SELECT * FROM users WHERE username = $1 AND password = $2",
        login_req.username,
        login_req.password 
    )
    .fetch_optional(&**db_pool)
    .await;

    match user_result {
        Ok(Some(_user)) => {
            HttpResponse::Ok().json(LoginResponse { token: "fake_token".to_string() }) 
        },
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        },
    }
}

async fn update_user_profile(user_id: web::Path<i32>, profile_req: web::Json<UserProfileRequest>, db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match sqlx::query!(
        "UPDATE users SET bio = $1 WHERE id = $2",
        profile_req.bio,
        user_id.0
    )
    .execute(&**db_pool)
    .await {
        Ok(_) => HttpResponse::Ok().json(UserProfileResponse {
            id: user_id.0,
            username: "ExampleUser".to_string(),
            bio: profile_req.bio.clone(),
        }),
        Err(e) => {
            log::error!("Failed to execute update query: {:?}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

async fn validate_token(req: web::HttpRequest) -> Result<(), actix_web::Error> {
    let token = req.headers().get("Authorization").and_then(|v| v.to_str().ok());

    match token {
        Some(t) if t == "Bearer fake_token" => Ok(()),
        _ => Err(actix_web::error::ErrorUnauthorized("Invalid Token")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::resource("/login")
                    .route(web::post().to(login))
            )
            .service(
                web::resource("/user/{id}/profile")
                    .route(web::put().to(update_user_profile))
                    .wrap_fn(|req, srv| {
                        validate_token(req).and_then(move |_| srv.call(req))
                    })
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}