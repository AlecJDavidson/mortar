use serde::{Deserialize, Serialize};

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
    pub id: String,
    pub name: String,
    pub creation_time: String, // DateTime<Utc>,
    pub last_invocation: String, // Option<DateTime<Utc>>,
    pub language: Language,
    pub source_path: String,
    pub active: bool
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    Rust,
    C,
    Bash,
}
