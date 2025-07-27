use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use sqlx::{query, query_as, Error as SqlxError};

use crate::{
    model::{BrickModel, BrickModelResponse},
    schema::{CreateBrickSchema, FilterOptions, UpdateBrickSchema},
    AppState,
};

// Convert DB Model to Response
fn to_brick_response(brick: &BrickModel) -> BrickModelResponse {
    BrickModelResponse {
        id: brick.id.to_owned(),
        name: brick.name.to_owned(),
        language: brick.language.to_owned(),
        source_path: brick.source_path.to_owned(),
        created_at: brick.created_at,
        last_invoked: brick.last_invoked,
    }
}

// Get all brick

pub async fn list_brick_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // Param
    let Query(_opts) = opts.unwrap_or_default();

    // Query without macro
    let bricks = query_as!(BrickModel, r#"SELECT * FROM bricks ORDER by id"#,)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    // Response
    let brick_responses = bricks
        .iter()
        .map(|brick| to_brick_response(&brick))
        .collect::<Vec<BrickModelResponse>>();

    let json_response = json!({
        "status": "ok",
        "count": brick_responses.len(),
        "bricks": brick_responses
    });

    Ok(Json(json_response))
}

// Create a brick

pub async fn create_brick_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateBrickSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // Insert
    let id = uuid::Uuid::new_v4().to_string();

    let query_result =
        query(r#"INSERT INTO bricks (id, name, language, source_path ) VALUES ($1, $2, $3, $4)"#)
            .bind(&id.to_string())
            .bind(&body.name)
            .bind(&body.language)
            .bind(&body.source_path)
            .execute(&data.db)
            .await
            .map_err(|err: SqlxError| err.to_string());

    // Duplicate err check
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = json!({
                "status": "error",
                "message": "Brick already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    // Get inserted brick by ID
    let brick = query_as!(BrickModel, r#"SELECT * FROM bricks WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let brick_response = json!({
        "status": "success",
        "data": json!({
            "brick": to_brick_response(&brick)
        })
    });

    Ok(Json(brick_response))
}

// Get a single Brick

pub async fn get_brick_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // Execute the query to fetch a brick by its ID
    let query_result = query_as!(BrickModel, r#"SELECT * FROM bricks WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(brick) => {
            // Construct the response if the brick is found
            let brick_response = json!({
                "status": "success",
                "data": json!({
                    "brick": to_brick_response(&brick)
                })
            });
            println!("{:?}", brick_response);
            Ok(Json(brick_response))
        }
        Err(SqlxError::RowNotFound) => {
            // Construct the error response if no brick is found
            let error_response = json!({
                "status": "fail",
                "message": format!("Brick with ID: {} not found", id)
            });
            println!("{:?}", error_response);
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => {
            // Construct the error response for any other SQLx errors
            let error_response = json!({
                "status": "error",
                "message": format!("{:?}", e),
            });
            println!("{:?}", error_response);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

// Update a brick

pub async fn update_brick_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateBrickSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // Validate if the brick exists
    let query_result = query_as!(BrickModel, r#"SELECT * FROM bricks WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await;
    // Fetch the result and handle errors
    let brick = match query_result {
        Ok(p) => p,
        Err(SqlxError::RowNotFound) => {
            let error_response = json!({
                "status": "error",
                "message": format!("Brick with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            let error_response = json!({
                "status": "error",
                "message": format!("{:?}", e)
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    // Update (if empty, use old value)
    let update_query =
        r#"UPDATE bricks SET name = $1, language = $2, source_path = $3 WHERE id = $4"#;
    let updated_name = body.name.unwrap_or(brick.name);
    let updated_language = body.language.unwrap_or(brick.language);
    let updated_source_path = body.source_path.unwrap_or(brick.source_path);

    let update_result = query(update_query)
        .bind(updated_name)
        .bind(updated_language) // Fixed variable name here
        .bind(updated_source_path)
        .bind(&id)
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                })),
            )
        })?;

    // Check if any rows were affected by the update
    if update_result.rows_affected() == 0 {
        let error_response = json!({
            "status": "error",
            "message": format!("Brick with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    // Return success response
    Ok(Json(json!({
        "status": "success",
        "message": format!("Brick with ID: {} updated successfully", id),
    })))
}

// Delete a brick

pub async fn delete_brick_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // delete with query macro
    let query_result = query!(r#"DELETE FROM bricks WHERE id = $1"#, &id)
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;

    // response
    if query_result.rows_affected() == 0 {
        let error_response = json!({
            "status": "error",
            "message": format!("Brick with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::OK)
}

// Invoke a brick
