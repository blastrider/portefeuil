use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/courses", web::post().to(handlers::add_course))
            .route("/courses", web::get().to(handlers::get_courses))
            .route("/courses/{id}", web::delete().to(handlers::delete_course))
            .route("/health", web::get().to(handlers::health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
