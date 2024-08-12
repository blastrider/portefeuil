mod config;
mod handlers;
mod middleware;
mod models;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use config::load_config;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::handlers::{
    add_course, delete_course, get_courses, health_check, login_user, register_user,
};
use crate::middleware::auth::jwt_middleware;

#[derive(Debug, serde::Deserialize)]
struct AppConfig {
    database_url: String,
    server_port: u16,
    log_level: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let app_config: AppConfig = config.try_deserialize().unwrap();

    env_logger::Builder::from_env(Env::default().default_filter_or(&app_config.log_level)).init();
    // charge la configuration

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&app_config.database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(jwt_middleware);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000") // Autoriser les requêtes du front-end
                    .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                    .wrap(auth) // Middleware JWT appliqué sur ces routes seulement
                    .route("/courses", web::get().to(get_courses))
                    .route("/courses", web::post().to(add_course))
                    .route("/courses/{id}", web::delete().to(delete_course))
                    .route(
                        "/check-repair-ids",
                        web::get().to(handlers::check_and_repair_ids),
                    ),
            )
            .route("/register", web::post().to(register_user)) // Route non protégée
            .route("/login", web::post().to(login_user)) // Route non protégée
            .route("/health", web::get().to(health_check)) // Route non protégée
    })
    .bind(("0.0.0.0", app_config.server_port))?
    .run()
    .await
}
