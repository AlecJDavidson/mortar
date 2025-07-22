use axum::{extract::Json, extract::State, http::StatusCode, response::IntoResponse};
// use serde_json::json;
use std::process::Command;
use std::sync::Arc;

use crate::structs::{Brick, Db, ErrorResponse, SuccessResponse};

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

pub async fn create_brick(
    State(state): State<Arc<Db>>,
    Json(payload): Json<Brick>,
) -> (axum::http::StatusCode, Json<Brick>) {
    let query = "INSERT INTO bricks (name, language, source_path, active) VALUES ($1, $2, $3, $4) RETURNING id";

    let brick_id: i32 = sqlx::query_scalar(query)
        .bind(&payload.name)
        .bind(&payload.language.to_string()) // Convert Language enum to string
        .bind(&payload.source_path)
        .bind(&payload.active)
        .fetch_one(&state.pool)
        .await
        .expect("Failed to insert brick");

    let brick = Brick {
        id: brick_id,
        name: payload.name,
        creation_time: None,
        last_invocation: None,
        language: payload.language,
        source_path: payload.source_path,
        active: true,
    };
    (axum::http::StatusCode::CREATED, Json(brick))
}
