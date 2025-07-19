use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

mod handlers;
mod structs;
use crate::handlers::{bad_request, create_resource, hello_world, not_found_handler};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello_world)) // just a test endpoint so I don't go insane
        .route("/create", post(create_resource)) // unused for now
        .route("/bad-request", get(bad_request))
        .fallback(get(not_found_handler));

    // Run it on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
