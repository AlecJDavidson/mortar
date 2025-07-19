use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::process::Command;

use crate::structs::{ErrorResponse, SuccessResponse, Brick};

pub async fn hello_world() -> impl IntoResponse {
    let success_response = SuccessResponse {
        status: StatusCode::OK.to_string(),
        message: "Hello, World!".to_string(),
    };
    (StatusCode::OK, Json(success_response)).into_response()
}

pub async fn create_resource(Json(_): Json<serde_json::Value>) -> impl IntoResponse {
    let success_response = SuccessResponse {
        status: StatusCode::CREATED.to_string(),
        message: "Resource created".to_string(),
    };
    (StatusCode::CREATED, Json(success_response)).into_response()
}

pub async fn bad_request() -> impl IntoResponse {
    let error_response = ErrorResponse {
        status: StatusCode::BAD_REQUEST.to_string(),
        message: "Bad request".to_string(),
    };
    (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
}

pub async fn not_found_handler() -> impl IntoResponse {
    let error_response = ErrorResponse {
        status: StatusCode::BAD_REQUEST.to_string(),
        message: "Not Found".to_string(),
    };
    (StatusCode::NOT_FOUND, Json(error_response)).into_response()
}

pub async fn invoke_brick(Json(brick): Json<Brick>) -> impl IntoResponse {
    match brick.language {
        crate::structs::Language::Rust | crate::structs::Language::C => {
            let output = Command::new(&brick.source_path)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let success_response = SuccessResponse {
                    status: StatusCode::OK.to_string(),
                    message: String::from_utf8_lossy(&output.stdout).to_string(),
                };
                (StatusCode::OK, Json(success_response)).into_response()
            } else {
                let error_response = ErrorResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message: String::from_utf8_lossy(&output.stderr).to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
        crate::structs::Language::Bash => {
            let output = Command::new("bash")
                .arg(&brick.source_path)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let success_response = SuccessResponse {
                    status: StatusCode::OK.to_string(),
                    message: String::from_utf8_lossy(&output.stdout).to_string(),
                };
                (StatusCode::OK, Json(success_response)).into_response()
            } else {
                let error_response = ErrorResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message: String::from_utf8_lossy(&output.stderr).to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}
