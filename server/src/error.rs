use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

/// Todos los errores salen como `{ "message": "..." }`, que es lo que lee el frontend.
pub enum AppError {
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict(String),
    Unavailable(String),
    TooMany,
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Inicia sesión para continuar.".into()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "No tienes permiso para esto.".into()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "No se encontró.".into()),
            AppError::Conflict(m) => (StatusCode::CONFLICT, m),
            AppError::Unavailable(m) => (StatusCode::SERVICE_UNAVAILABLE, m),
            AppError::TooMany => (StatusCode::TOO_MANY_REQUESTS, "Demasiados intentos. Espera un minuto e inténtalo de nuevo.".into()),
            AppError::Internal(m) => {
                tracing::error!("{m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Algo salió mal. Inténtalo de nuevo.".into())
            }
        };
        (code, Json(json!({ "message": msg }))).into_response()
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(e: tokio_postgres::Error) -> Self {
        // 23505 = violación de índice único (correo repetido, postulación repetida)
        if e.code().map(|c| c.code()) == Some("23505") {
            return AppError::Conflict("Ya existe.".into());
        }
        let detail = e.as_db_error().map(|d| format!("{} ({})", d.message(), d.code().code())).unwrap_or_else(|| e.to_string());
        AppError::Internal(format!("postgres: {detail}"))
    }
}
impl From<deadpool_postgres::PoolError> for AppError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        AppError::Internal(format!("pool: {e}"))
    }
}
