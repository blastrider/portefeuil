use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::{Course, NewCourse};

pub async fn add_course(pool: web::Data<PgPool>, course: web::Json<NewCourse>) -> impl Responder {
    let query =
        "INSERT INTO courses (name, amount, category, date) VALUES ($1, $2, $3, $4) RETURNING id";
    let id: (i32,) = sqlx::query_as(query)
        .bind(&course.name)
        .bind(course.amount)
        .bind(&course.category)
        .bind(course.date)
        .fetch_one(pool.get_ref())
        .await
        .expect("Failed to insert course.");

    HttpResponse::Ok().json(id)
}

pub async fn get_courses(pool: web::Data<PgPool>) -> impl Responder {
    let courses = sqlx::query_as::<_, Course>("SELECT * FROM courses")
        .fetch_all(pool.get_ref())
        .await
        .expect("Failed to fetch courses.");

    HttpResponse::Ok().json(courses)
}
/* pub async fn delete_course(pool: web::Data<PgPool>, course_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM courses WHERE id = $1", *course_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Course deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete course"),
    }
} */

pub async fn delete_course(pool: web::Data<PgPool>, course_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as::<_, (i32,)>("DELETE FROM courses WHERE id = $1 RETURNING id")
        .bind(*course_id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Course deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete course"),
    }
}

pub async fn health_check(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query("SELECT 1").fetch_one(pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("API is running and database is connected"),
        Err(_) => HttpResponse::InternalServerError()
            .body("API is running but failed to connect to the database"),
    }
}
