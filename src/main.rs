use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;
use dotenvy::dotenv;
use tracing_subscriber;

mod db;
mod routes;
mod models;
mod handlers;
mod services;
mod utils;

use db::connect_to_db;
use routes::auth::auth_routes;
use routes::tickets::ticket_routes;
use routes::notes::note_routes;
use routes::collaboration::collaboration_routes;
use routes::knowledge_base::knowledge_base_routes;
use routes::reports::report_routes; // ✅ NEW for Reports

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Connect to PostgreSQL using SQLx
    let db_pool = connect_to_db().await;

    // Build Axum app with all routes
    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/auth", auth_routes().with_state(db_pool.clone()))
        .nest("/tickets", ticket_routes().with_state(db_pool.clone()))
        .nest("/notes", note_routes().with_state(db_pool.clone()))
        .nest("/collab", collaboration_routes().with_state(db_pool.clone()))
        .nest("/kb", knowledge_base_routes().with_state(db_pool.clone()))
        .nest("/reports", report_routes().with_state(db_pool.clone())) // ✅ Add report routes
        .with_state(db_pool); // Global app-wide state

    // Start server
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");
    println!("🚀 Server running at http://127.0.0.1:3000");

    serve(listener, app).await.unwrap();
}

// Root GET /
async fn root_handler() -> &'static str {
    "Customer Support API is running 🚀"
}
