use axum::{Router, routing::get};

mod authentication;
mod db;
mod models;
mod routes;
mod services;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().ok();

    // init tracing
    tracing_subscriber::fmt::init();

    // connect to mongodb
    let db = db::MongoDb::new()
        .await
        .expect("Failed to connect to MongoDB");
    let app = routes::create_router(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to connect to server");

    tracing::info!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Error serving mothrbox");

    // println!("Listening on {}", listener.local_addr().unwrap());
}
