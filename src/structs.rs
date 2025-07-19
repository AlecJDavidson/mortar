use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub status: String,
    pub message: String,
}
