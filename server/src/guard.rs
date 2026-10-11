//! Lo que protege a la API cuando está en internet: límite de intentos por IP (contraseñas y gasto de IA),
//! cabeceras de seguridad y de caché.

use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use axum::{
    extract::{Request, State},
    http::{header, HeaderValue},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::error::AppError;

/// Ventana fija por IP. ponytail: en memoria y por proceso; con varias instancias el límite real se multiplica.
pub struct Limiter {
    hits: Mutex<HashMap<String, (u32, Instant)>>,
    max: u32,
    window: Duration,
}

impl Limiter {
    pub fn new(max: u32, window: Duration) -> Self {
        Self { hits: Mutex::new(HashMap::new()), max, window }
    }

    pub fn allow(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut h = self.hits.lock().unwrap();
        if h.len() > 10_000 {
            h.retain(|_, (_, t)| now.duration_since(*t) < self.window); // evita que crezca sin límite
        }
        let e = h.entry(key.to_string()).or_insert((0, now));
        if now.duration_since(e.1) >= self.window {
            *e = (0, now);
        }
        e.0 += 1;
        e.0 <= self.max
    }
}

/// IP del cliente: Caddy agrega la real al final de `X-Forwarded-For`; lo que venga antes lo puede inventar cualquiera.
fn client_ip(req: &Request) -> String {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.rsplit(',').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "local".into())
}

pub async fn limit(State(l): State<std::sync::Arc<Limiter>>, req: Request, next: Next) -> Response {
    if !l.allow(&client_ip(&req)) {
        return AppError::TooMany.into_response();
    }
    next.run(req).await
}

/// Cabeceras de seguridad en todo, y caché: los archivos con huella (`/assets/…`) para siempre, el resto no se guarda.
pub async fn headers(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let mut res = next.run(req).await;
    let h = res.headers_mut();
    h.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    h.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    h.insert(header::REFERRER_POLICY, HeaderValue::from_static("strict-origin-when-cross-origin"));
    h.insert(header::STRICT_TRANSPORT_SECURITY, HeaderValue::from_static("max-age=31536000"));
    if !h.contains_key(header::CACHE_CONTROL) {
        let v = if path.starts_with("/assets/") { "public, max-age=31536000, immutable" } else { "no-cache" };
        h.insert(header::CACHE_CONTROL, HeaderValue::from_static(v));
    }
    res
}
