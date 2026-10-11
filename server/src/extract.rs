//! Extractores que, si la petición viene mal, responden `{ "message": … }` como el resto de la API
//! (los de Axum devuelven texto plano, que el frontend no sabe mostrar).

use axum::{
    extract::{FromRequest, FromRequestParts, Path, Query, Request},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::error::AppError;

pub struct J<T>(pub T);
pub struct P<T>(pub T);
pub struct Q<T>(pub T);

impl<T: DeserializeOwned, S: Send + Sync> FromRequest<S> for J<T> {
    type Rejection = AppError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<T>::from_request(req, state)
            .await
            .map(|axum::Json(v)| J(v))
            .map_err(|_| AppError::BadRequest("Revisa los datos enviados: falta algo o tiene un formato incorrecto.".into()))
    }
}

impl<T: DeserializeOwned + Send, S: Send + Sync> FromRequestParts<S> for P<T> {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::<T>::from_request_parts(parts, state).await.map(|Path(v)| P(v)).map_err(|_| AppError::BadRequest("Identificador no válido.".into()))
    }
}

impl<T: DeserializeOwned, S: Send + Sync> FromRequestParts<S> for Q<T> {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Query::<T>::from_request_parts(parts, state).await.map(|Query(v)| Q(v)).map_err(|_| AppError::BadRequest("Los filtros de búsqueda no son válidos.".into()))
    }
}
