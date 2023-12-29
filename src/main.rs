use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Handler for /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}
// Handler for /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Paul".to_string(),
            email: "admin@paul-codes.com".to_string(),
        },
        User {
            id: 2,
            name: "Lucy".to_string(),
            email: "lucy@mail.com".to_string(),
        },
        User {
            id: 3,
            name: "Jennifer".to_string(),
            email: "jennifer@mail.com".to_string(),
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users));

    println!("Running on http://localhost:3000");
    // Start server
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
