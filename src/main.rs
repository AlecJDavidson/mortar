use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

// mod db;
mod handlers;
mod structs;

use crate::handlers::{
    bad_request, create_brick, create_resource, hello_world, invoke_brick, not_found_handler,
};
use crate::structs::Db;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the database");

    let db = Db::new(pool).await.expect("Failed to create database pool");
    let state = Arc::new(db);

    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/create", post(create_resource))
        .route("/bad-request", get(bad_request))
        .route("/invoke-brick", post(invoke_brick))
        .route("/create-brick", post(create_brick))
        .with_state(state)
        .fallback(not_found_handler);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
