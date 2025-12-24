// src/services/mod.rs
use crate::authentication::{
    AuthError, AuthResponse, LoginRequest, RegisterRequest, generate_token, hash_password,
    verify_password,
};
use crate::db::MongoDb;
use crate::models::{PublicUser, User};
use axum::Json;
use mongodb::bson::doc;

// ==================== Authentication Services ====================

pub async fn authenticate_user(
    db: &MongoDb,
    login: LoginRequest,
) -> Result<Json<AuthResponse>, AuthError> {
    let collection = db.users_collection();

    // Find user by email
    let user = collection
        .find_one(doc! { "email": &login.email })
        .await
        .map_err(|e| {
            tracing::error!("Database error during login: {}", e);
            AuthError::DatabaseError
        })?
        .ok_or(AuthError::InvalidCredentials)?;

    // Verify password
    if !verify_password(&login.password, &user.password_hash)? {
        return Err(AuthError::InvalidCredentials);
    }

    // Generate JWT token
    let token = generate_token(&user.id_string(), &user.email)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn register_user(
    db: &MongoDb,
    register: RegisterRequest,
) -> Result<Json<AuthResponse>, AuthError> {
    let collection = db.users_collection();

    // Check if user already exists
    let existing = collection
        .find_one(doc! { "email": &register.email })
        .await
        .map_err(|e| {
            tracing::error!("Database error checking existing user: {}", e);
            AuthError::DatabaseError
        })?;

    if existing.is_some() {
        return Err(AuthError::UserAlreadyExists);
    }

    // Hash password
    let password_hash = hash_password(&register.password)?;

    // Create new user
    let mut new_user = User::new(register.email, password_hash);

    // Insert into database
    let result = collection.insert_one(&new_user).await.map_err(|e| {
        tracing::error!("Database error inserting user: {}", e);
        AuthError::DatabaseError
    })?;

    // Set the generated ID on our user object
    new_user.id = result.inserted_id.as_object_id();

    // Generate JWT token
    let token = generate_token(&new_user.id_string(), &new_user.email)?;

    Ok(Json(AuthResponse {
        token,
        user: new_user.into(),
    }))
}

// ==================== User Services ====================

pub async fn get_user_by_id(db: &MongoDb, user_id: &str) -> Result<PublicUser, AuthError> {
    let collection = db.users_collection();

    let oid =
        mongodb::bson::oid::ObjectId::parse_str(user_id).map_err(|_| AuthError::InvalidToken)?;

    let user = collection
        .find_one(doc! { "_id": oid })
        .await
        .map_err(|e| {
            tracing::error!("Database error fetching user: {}", e);
            AuthError::DatabaseError
        })?
        .ok_or(AuthError::InvalidCredentials)?;

    Ok(user.into())
}
