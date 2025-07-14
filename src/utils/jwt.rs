use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::env;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,     // user ID
    pub name: String,    // ✅ name added
    pub role: String,
    pub exp: usize,
}

pub fn generate_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        name: user.name.clone(),  // ✅ include user name
        role: user.role.clone(),
        exp: expiration as usize,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
