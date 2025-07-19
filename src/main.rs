use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

mod structs;
mod handlers;
use crate::handlers::{bad_request, create_resource, hello_world, not_found_handler};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/create", post(create_resource))
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
