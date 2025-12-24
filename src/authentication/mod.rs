use axum::{
    Json,
    extract::FromRequestParts,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

pub struct AuthUser {
    pub id: String,
    pub email: String,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    MissingToken,
    ExpiredToken,
    InvalidCredentials,
    UserAlreadyExists,
    DatabaseError,
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token expired"),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AuthError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (
            status,
            Json(ErrorResponse {
                error: message.to_string(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// ==================== Request/Response DTOs ====================

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: crate::models::PublicUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: usize, // expiration timestamp
    pub iat: usize, // issued at timestamp
}

fn get_jwt_secret() -> String {
    env::var("key").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set, using default (NOT SECURE FOR PRODUCTION)");
        "your-secret-key-change-this-in-production".to_string()
    })
}

pub fn generate_token(user_id: &str, email: &str) -> Result<String, AuthError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .ok_or(AuthError::InternalError)?
        .timestamp() as usize;

    let issued_at = chrono::Utc::now().timestamp() as usize;

    let claim = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
        iat: issued_at,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("Failed to generate token: {}", e);
        AuthError::InternalError
    })
}

pub fn validate_token(token: &str) -> Result<Claims, AuthError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| {
        tracing::debug!("Token validation failed: {}", e);
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
            _ => AuthError::InvalidToken,
        }
    })
}

// ==================== Extractor ====================

// #[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        // Extract Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        // Validate token and extract claims
        let claims = validate_token(token)?;

        Ok(AuthUser {
            id: claims.sub,
            email: claims.email,
        })
    }
}

// ==================== Password Hashing ====================
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
        tracing::error!("Failed to hash password: {}", e);
        AuthError::InternalError
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    bcrypt::verify(password, hash).map_err(|e| {
        tracing::error!("Failed to verify password: {}", e);
        AuthError::InternalError
    })
}
