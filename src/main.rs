// src/main.rs
mod authentication;
mod db;
mod models;
mod routes;
mod services;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file (only works locally)
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    tracing::info!("Starting Mothrbox Backend V2...");

    // Get configuration from environment
    let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
        tracing::warn!("MONGODB_URI not set, using default");
        "mongodb://localhost:27017".to_string()
    });

    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| {
        tracing::warn!("DATABASE_NAME not set, using default 'mothrbox'");
        "mothrbox".to_string()
    });

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set, using default (INSECURE!)");
        "default-secret-key".to_string()
    });

    tracing::info!(
        "MongoDB URI: {}",
        mongodb_uri.split('@').last().unwrap_or("hidden")
    );
    tracing::info!("Database: {}", database_name);
    tracing::info!(
        "JWT Secret: {}",
        if jwt_secret.len() > 10 {
            "✓ Set"
        } else {
            "⚠ Too short"
        }
    );

    // Connect to MongoDB
    tracing::info!("Connecting to MongoDB...");
    let db = match db::MongoDb::new().await {
        Ok(db) => {
            tracing::info!("✓ Successfully connected to MongoDB");
            db
        }
        Err(e) => {
            tracing::error!("✗ Failed to connect to MongoDB: {}", e);
            tracing::error!("Make sure MONGODB_URI is set correctly");
            std::process::exit(1);
        }
    };

    // Create the router with database state
    let app = routes::create_router(db);

    // Get port from environment (Koyeb sets PORT)
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let addr = format!("{}:{}", host, port);

    tracing::info!("Starting server on {}", addr);

    // Start server
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            tracing::info!("✓ Server listening on {}", addr);
            listener
        }
        Err(e) => {
            tracing::error!("✗ Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    tracing::info!("🚀 Mothrbox Backend V2 is ready!");
    tracing::info!("📡 Health check: http://{}/health", addr);

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}

/*
================================================================================
                        MOTHRBOX BACKEND V2 - API DOCUMENTATION
================================================================================

ENVIRONMENT VARIABLES (Required for Koyeb):
-------------------------------------------
MONGODB_URI=mongodb+srv://username:password@cluster.mongodb.net/mothrbox?retryWrites=true&w=majority
DATABASE_NAME=mothrbox
JWT_SECRET=your-super-secret-jwt-key-change-this
PORT=8000 (automatically set by Koyeb)

================================================================================

API ENDPOINTS:

PUBLIC ENDPOINTS:
-----------------
GET  /health              - Health check endpoint
POST /auth/register       - Register a new user
POST /auth/login          - Login and get JWT token

PROTECTED ENDPOINTS (require Bearer token):
--------------------------------------------
GET  /auth/profile        - Get current user's profile
GET  /protected          - Example protected endpoint

================================================================================

KOYEB DEPLOYMENT:

1. Set up MongoDB Atlas (free tier):
   - Go to https://cloud.mongodb.com
   - Create a cluster
   - Get connection string
   - Whitelist IP: 0.0.0.0/0 (allow all)

2. In Koyeb, set environment variables:
   - MONGODB_URI: your-mongodb-atlas-connection-string
   - DATABASE_NAME: mothrbox
   - JWT_SECRET: generate-a-secure-random-string

3. Deploy from GitHub or Docker

4. Koyeb will automatically set PORT environment variable

================================================================================

EXAMPLE USAGE:

1. Register a new user:
   curl -X POST https://your-app.koyeb.app/auth/register \
     -H "Content-Type: application/json" \
     -d '{
       "email": "user@example.com",
       "username": "johndoe",
       "password": "securepassword123"
     }'

2. Login:
   curl -X POST https://your-app.koyeb.app/auth/login \
     -H "Content-Type: application/json" \
     -d '{
       "email": "user@example.com",
       "password": "securepassword123"
     }'

3. Access protected route:
   curl https://your-app.koyeb.app/auth/profile \
     -H "Authorization: Bearer YOUR_JWT_TOKEN"

================================================================================
*/
