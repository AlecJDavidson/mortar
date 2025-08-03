use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/* ### Bricks ### */

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct BrickModel {
    pub id: String,
    pub name: String,
    pub language: String,
    pub source_path: String,

    pub created_at: Option<NaiveDateTime>,
    pub last_invoked: Option<NaiveDateTime>,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BrickModelResponse {
    pub id: String,
    pub name: String,
    pub language: String,
    pub source_path: String,

    pub created_at: Option<NaiveDateTime>,
    pub last_invoked: Option<NaiveDateTime>,
}

/* ### Logging ### */

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct LogModel {
    pub id: String,
    pub name: String,
    pub language: String,
    pub source_path: String,

    pub created_at: Option<NaiveDateTime>,
    pub last_invoked: Option<NaiveDateTime>,
}
