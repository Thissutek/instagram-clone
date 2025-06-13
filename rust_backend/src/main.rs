use axum::{
    extract::Path,
    response::Json,
    routing::get, // Removed unused 'post' import
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// Rust Concept: Structs with derive macros
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
    bio: Option<String>, // Option<T> means this field can be None
}

// Rust Concept: Type aliases make code cleaner
type UserId = u32;
type UserDatabase = HashMap<UserId, User>;

// Rust Concept: This attribute makes async main() possible
#[tokio::main]
async fn main() {
    println!("🚀 Starting Instagram Backend...");

    // Create our application router
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check));

    // Create a TCP listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("📡 Server running on http://127.0.0.1:3000");

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

// Handler function for the root route
async fn hello_world() -> Json<Value> {
    Json(json!({
        "message": "Welcome to Instagram Backend!",
        "status": "running",
        "endpoints": ["/users", "/users/:id", "/health"]
    }))
}

// Health check endpoint
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "instagram_backend"
    }))
}

// Rust Concept: This function shows a problem we need to solve!
async fn get_all_users() -> Json<Vec<User>> {
    // Problem: We can't access the users HashMap from main()
    // This is where we'll learn about Rust's state management
    let empty_users = Vec::new();
    Json(empty_users)
}

// Rust Concept: Path extraction and error handling
async fn get_user_by_id(Path(id): Path<u32>) -> Json<Value> {
    // For now, return a placeholder
    Json(json!({
        "message": format!("Looking for user with ID: {}", id),
        "note": "We'll implement this properly next!"
    }))
}
