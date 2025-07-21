use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::{env, net::SocketAddr, sync::Arc};

mod db;
mod handlers;
mod structs;
use crate::handlers::{
    bad_request, create_brick, create_resource, hello_world, invoke_brick, not_found_handler,
};
use crate::structs::Db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the database");

    let db = Db::new(&database_url)
        .await
        .expect("Failed to create database pool");
    // let state = Arc::new(db);

    let app = Router::new()
        .route("/hello", get(hello_world)) // just a test endpoint so I don't go insane
        .route("/create", post(create_resource)) // unused for now
        .route("/bad-request", get(bad_request))
        .route("/invoke-brick", post(invoke_brick)) // brick invocation
        .route("/create-brick", post(create_brick)) // brick creation
        // .layer(FromRef::default().into_shared_state(state))
        .fallback(get(not_found_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
