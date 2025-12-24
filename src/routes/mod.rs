// src/routes/mod.rs
use crate::authentication::{AuthError, AuthResponse, AuthUser, LoginRequest, RegisterRequest};
use crate::db::MongoDb;
use crate::models::PublicUser;
use crate::services;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};

// ==================== Auth Routes ====================

pub async fn login(
    State(db): State<MongoDb>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    services::authenticate_user(&db, payload).await
}

pub async fn register(
    State(db): State<MongoDb>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    services::register_user(&db, payload).await
}

pub async fn get_profile(
    State(db): State<MongoDb>,
    user: AuthUser,
) -> Result<Json<PublicUser>, AuthError> {
    let public_user = services::get_user_by_id(&db, &user.id).await?;
    Ok(Json(public_user))
}

// ==================== Protected Routes ====================

pub async fn protected_example(user: AuthUser) -> String {
    format!("Hello, {}! This is a protected route.", user.email)
}

// ==================== Public Routes ====================

pub async fn health_check() -> &'static str {
    "OK"
}

// ==================== Router Setup ====================

pub fn create_router(db: MongoDb) -> Router {
    Router::new()
        // Public routes
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        // Protected routes (require authentication)
        .route("/auth/profile", get(get_profile))
        .route("/protected", get(protected_example))
        // Add MongoDB state to all routes
        .with_state(db)
}
