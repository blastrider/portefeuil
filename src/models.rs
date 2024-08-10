use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub amount: f64,
    pub category: String,
    pub date: chrono::NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct NewCourse {
    pub name: String,
    pub amount: f64,
    pub category: String,
    pub date: chrono::NaiveDate,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
