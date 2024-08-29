use axum::extract::{Json, Path};
use axum::{response::IntoResponse, routing::get, serve, Router};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    email: String,
    active: bool,
}

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/", get(root))
        .route("/user", get(fetch_user))
        .route("/user/:id", get(fetch_user_by_id));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on localhost:3000");
    serve(listener, routes).await.unwrap();
}

async fn fetch_user() -> impl IntoResponse {
    Json(User {
        id: 1,
        username: String::from("rust_user"),
        email: String::from("rust_user@example.com"),
        active: true,
    })
}

async fn fetch_user_by_id(Path(user_id): Path<u64>) -> impl IntoResponse {
    Json(User {
        id: user_id,
        username: String::from("rust_user"),
        email: String::from("rust_user@example.com"),
        active: true,
    })
}

async fn root() -> impl IntoResponse {
    "Hello World! from Axum Web Serve"
}
