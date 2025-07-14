use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,     // ✅ name added here too
    pub role: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: String,
    pub name: String,     // ✅ expose name to handlers
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String); // ✅ required by Axum 0.7

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()))?;

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(AuthUser {
            user_id: decoded.claims.sub,
            name: decoded.claims.name, // ✅ decode name
            role: decoded.claims.role,
        })
    }
}
