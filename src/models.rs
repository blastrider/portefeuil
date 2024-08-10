use serde::{Deserialize, Serialize};

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

