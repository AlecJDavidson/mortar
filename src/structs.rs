use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::fmt;
use uuid::Uuid;

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

#[derive(Serialize, Deserialize)]
pub struct Brick {
    pub id: Option<Uuid>,
    pub name: String,
    pub creation_time: Option<String>,   // DateTime<Utc>,
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

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust"),
            Language::C => write!(f, "C"),
            Language::Bash => write!(f, "Bash"),
        }
    }
}

pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(pool: Pool<Postgres>) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}
