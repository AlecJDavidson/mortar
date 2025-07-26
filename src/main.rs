/* ### SETUP THE SERVER ### */

/* ALLOW DEAD CODE */
#[allow(dead_code)]
use axum::{
    response::IntoResponse,
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use dotenv::dotenv;
use handler::{
    create_brick_handler, delete_brick_handler, get_brick_handler, list_brick_handler,
    update_brick_handler,
};
use tokio::net::TcpListener;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

mod handler;
mod model;
mod schema;
mod structs;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Read from .env file

    // Setup the DB Connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/api/hello", get(hello_handler))
        .route("/api/healthcheck", get(health_check_handler))
        .route("/api/brick", post(create_brick_handler))
        .route("/api/brick", get(list_brick_handler))
        .route("/api/brick/:id", get(get_brick_handler))
        .route("/api/brick/:id", put(update_brick_handler))
        .route("/api/brick/:id", patch(update_brick_handler))
        .route("/api/brick/:id", delete(delete_brick_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    println!("✅ Server started successfully at 0.0.0.0:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn hello_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Hello World!";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
