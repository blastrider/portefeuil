use actix_web::{web, HttpResponse, Responder};
use argon2::{
    self,
    password_hash::{rand_core, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgConnection, PgPool};
use uuid::Uuid;

use crate::models::LoginUser;
use crate::models::{Course, NewCourse, RegisterUser};

#[derive(Deserialize)]
pub struct CourseFilter {
    name: Option<String>,
    category: Option<String>,
    date: Option<String>,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub sub: String,
    pub exp: usize,
}

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

pub async fn get_courses(
    pool: web::Data<PgPool>,
    filter: web::Query<CourseFilter>,
) -> impl Responder {
    let mut query = String::from("SELECT * FROM courses WHERE 1=1");

    if let Some(name) = &filter.name {
        query.push_str(&format!(" AND name LIKE '%{}%'", name));
    }

    if let Some(category) = &filter.category {
        query.push_str(&format!(" AND category = '{}'", category));
    }

    if let Some(date) = &filter.date {
        query.push_str(&format!(" AND date = '{}'", date));
    }

    if let Some(min_amount) = filter.min_amount {
        query.push_str(&format!(" AND amount >= {}", min_amount));
    }

    if let Some(max_amount) = filter.max_amount {
        query.push_str(&format!(" AND amount <= {}", max_amount));
    }

    let courses = query_as::<_, Course>(&query)
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

pub async fn register_user(
    pool: web::Data<PgPool>,
    user_data: web::Json<RegisterUser>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_password = argon2
        .hash_password(user_data.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, hashed_password, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        user_data.username,
        user_data.email,
        hashed_password,
        Utc::now().naive_utc(),
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to register user: {}", e))
        }
    }
}

pub async fn login_user(
    pool: web::Data<PgPool>,
    user_data: web::Json<LoginUser>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"SELECT id, username, email, hashed_password, created_at FROM users WHERE email = $1"#,
        user_data.email
    )
    .fetch_one(pool.get_ref())
    .await;

    let user = match result {
        Ok(record) => record,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid credentials"),
    };

    let parsed_hash = PasswordHash::new(&user.hashed_password).unwrap();
    let argon2 = Argon2::default();

    let is_valid = argon2
        .verify_password(user_data.0.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(token)
}
