use axum::Router;
use tokio::net::TcpListener;

mod authentication;
mod db;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // --- Read PORT from Koyeb ---
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = format!("0.0.0.0:{port}");

    // --- MongoDB ---
    let db = db::MongoDb::new()
        .await
        .expect("Failed to connect to MongoDB");

    let app = routes::create_router(db);

    // --- Bind ---
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");

    tracing::info!("🚀 Server running on http://{}", addr);

    // --- Serve (BLOCKS forever) ---
    axum::serve(listener, app)
        .await
        .expect("Axum server crashed");
}
