use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::FromRequestParts, http::request::Parts};
use rand_core::OsRng;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppError, AppState};

pub struct Keys {
    enc: EncodingKey,
    dec: DecodingKey,
}
impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self { enc: EncodingKey::from_secret(secret), dec: DecodingKey::from_secret(secret) }
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    role: String,
    exp: usize,
}

pub fn token(keys: &Keys, id: Uuid, role: &str) -> Result<String, AppError> {
    let exp = (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize;
    encode(&Header::default(), &Claims { sub: id, role: role.to_string(), exp }, &keys.enc).map_err(|e| AppError::Internal(format!("jwt: {e}")))
}

/// Argon2id es lento a propósito: va en un hilo aparte para no frenar a los demás.
pub async fn hash(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map(|h| h.to_string())
            .map_err(|e| AppError::Internal(format!("argon2: {e}")))
    })
    .await
    .map_err(|e| AppError::Internal(format!("join: {e}")))?
}

pub async fn verify(password: String, stored: String) -> bool {
    tokio::task::spawn_blocking(move || {
        PasswordHash::new(&stored).map(|h| Argon2::default().verify_password(password.as_bytes(), &h).is_ok()).unwrap_or(false)
    })
    .await
    .unwrap_or(false)
}

/// Quién hace la petición, sacado del `Authorization: Bearer …` (sin tocar la base).
pub struct Auth {
    pub id: Uuid,
    pub role: String,
}

impl Auth {
    pub fn employer(&self) -> Result<(), AppError> {
        if self.role == "employer" { Ok(()) } else { Err(AppError::Forbidden) }
    }
    pub fn student(&self) -> Result<(), AppError> {
        if self.role == "student" { Ok(()) } else { Err(AppError::Forbidden) }
    }
}

impl FromRequestParts<AppState> for Auth {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let h = parts.headers.get("authorization").and_then(|v| v.to_str().ok()).ok_or(AppError::Unauthorized)?;
        let t = h.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;
        let data = decode::<Claims>(t, &state.keys.dec, &Validation::default()).map_err(|_| AppError::Unauthorized)?;
        Ok(Auth { id: data.claims.sub, role: data.claims.role })
    }
}
