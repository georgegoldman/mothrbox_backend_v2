// src/db/mod.rs
use mongodb::{Client, Collection, Database};
use std::env;

#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub database: Database,
}

impl MongoDb {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let uri =
            env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

        let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "mothrbox".to_string());

        tracing::info!("Attempting to connect to MongoDB...");

        let client = Client::with_uri_str(&uri).await?;
        let database = client.database(&database_name);

        // Test the connection
        tracing::info!("Testing MongoDB connection...");
        database.run_command(bson::doc! { "ping": 1 }).await?;

        tracing::info!(
            "Successfully connected to MongoDB database: {}",
            database_name
        );

        Ok(Self { client, database })
    }

    pub fn users_collection(&self) -> Collection<crate::models::User> {
        self.database.collection("users")
    }
}

// Helper to pass MongoDB client through Axum state
pub type DbClient = MongoDb;
