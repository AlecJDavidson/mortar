use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
// use uuid::Uuid;

mod handlers;
mod structs;
use crate::handlers::{bad_request, create_resource, hello_world, invoke_brick, not_found_handler};
// use crate::structs::{Brick, Language};

#[tokio::main]
async fn main() {
    // Hardcoding a test brick
    // let test_brick = Brick {
    //     id: "n0nprod-00000001".to_string(),
    //     name: "Test Brick".to_string(),
    //     creation_time: "".to_string(),
    //     last_invocation: "".to_string(),
    //     language: Language::Bash,
    //     source_path: "../test_scripts/hello.sh".to_string(), // Update this path to a valid Bash script for testing
    //     active: true,
    // };

    let app = Router::new()
        .route("/hello", get(hello_world)) // just a test endpoint so I don't go insane
        .route("/create", post(create_resource)) // unused for now
        .route("/bad-request", get(bad_request))
        .route("/invoke-brick", post(invoke_brick)) // brick invocation
        .fallback(get(not_found_handler));

    // Run it on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
