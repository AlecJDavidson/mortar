use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
// use uuid::Uuid; // TODO: Get this working

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse {
    pub status: String,
    pub message: String,
}

// Old

// #[derive(Serialize, Deserialize)]
// pub struct Brick {
//     pub id: String,
//     pub name: String,
//     pub creation_time: String, // DateTime<Utc>,
//     pub last_invocation: String, // Option<DateTime<Utc>>,
//     pub language: Language,
//     pub source_path: String,
//     pub active: bool
// }

// New

#[derive(Serialize, Deserialize)]
pub struct Brick {
    pub id: String,
    pub name: String,
    pub creation_time: String,           // DateTime<Utc>,
    pub last_invocation: Option<String>, // Option<DateTime<Utc>>,
    pub language: Language,
    pub source_path: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    Rust,
    C,
    Bash,
}

pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(pool: Pool<Postgres>) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}
