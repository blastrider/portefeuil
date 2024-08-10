use actix_web::{web, HttpResponse, Responder};
use sqlx::{PgConnection, PgPool, Transaction};

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

pub async fn delete_course(pool: web::Data<PgPool>, course_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM courses WHERE id = $1")
        .bind(*course_id)
        .execute(pool.get_ref())
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

pub async fn check_and_repair_ids(pool: web::Data<PgPool>) -> impl Responder {
    let mut transaction = pool.begin().await.expect("Failed to start transaction");

    let result = sqlx::query_as::<_, (i32,)>("SELECT id FROM courses ORDER BY id ASC")
        .fetch_all(&mut *transaction)
        .await;

    match result {
        Ok(rows) => {
            let mut last_id = 0;
            let mut update_needed = false;

            for (index, row) in rows.iter().enumerate() {
                if row.0 != (index as i32 + 1) {
                    update_needed = true;
                    break;
                }
                last_id = row.0;
            }

            if update_needed {
                if let Err(err) = repair_id_sequence(&mut *transaction, last_id as i64).await {
                    transaction
                        .rollback()
                        .await
                        .expect("Failed to rollback transaction");
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to repair sequence: {}", err));
                } else {
                    transaction
                        .commit()
                        .await
                        .expect("Failed to commit transaction");
                    return HttpResponse::Ok().body("ID sequence was broken and has been repaired");
                }
            }

            HttpResponse::Ok().body("All IDs are consecutive, no repair needed")
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to query the database"),
    }
}

async fn repair_id_sequence(conn: &mut PgConnection, last_id: i64) -> Result<(), sqlx::Error> {
    let mut new_id = 1;

    let rows = sqlx::query_as::<_, (i32,)>("SELECT id FROM courses ORDER BY id ASC")
        .fetch_all(&mut *conn)
        .await?;

    for row in rows {
        if row.0 != new_id {
            sqlx::query("UPDATE courses SET id = $1 WHERE id = $2")
                .bind(new_id)
                .bind(row.0)
                .execute(&mut *conn)
                .await?;
        }
        new_id += 1;
    }

    // Reset sequence to the correct next id
    sqlx::query("SELECT setval(pg_get_serial_sequence('courses', 'id'), $1)")
        .bind(last_id)
        .execute(&mut *conn)
        .await?;

    Ok(())
}
