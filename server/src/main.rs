//! API de empleos: Axum + Tokio + Postgres (tokio-postgres con pool y sentencias preparadas).
//! Rápido por diseño: sin ORM, una sola consulta por endpoint, JSON tipado, mimalloc, LTO y un índice
//! semántico en RAM para la búsqueda con IA (sin extensiones de Postgres que instalar).

mod ai;
mod auth;
mod db;
mod error;
mod extract;
mod guard;
mod index;
mod jobs;
mod notifications;
mod users;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    middleware,
    routing::{any, get, post, put},
    Router,
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, services::{ServeDir, ServeFile}, trace::TraceLayer};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
    pub keys: Arc<auth::Keys>,
    pub ai: Option<Arc<ai::Gemini>>,
    pub index: Arc<index::VecIndex>,
}

fn env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(env("RUST_LOG", "info,tower_http=warn"))
        .init();

    let database_url = env("DATABASE_URL", "postgres://localhost/empleos");
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET no definido: usando uno de desarrollo (NO en producción)");
        "dev-secret-cambia-esto-en-produccion-0123456789".into()
    });

    let pool = db::connect(&database_url).await.expect("no se pudo conectar a Postgres");
    db::migrate(&pool).await.expect("fallaron las migraciones");

    let ai = std::env::var("GEMINI_API_KEY").ok().filter(|k| !k.is_empty()).map(|key| {
        Arc::new(ai::Gemini::new(key, env("GEMINI_MODEL", "gemini-3.5-flash-lite"), env("GEMINI_EMBED_MODEL", "gemini-embedding-001"), env("GEMINI_BASE", "https://generativelanguage.googleapis.com/v1beta/models")))
    });
    if ai.is_none() {
        tracing::warn!("GEMINI_API_KEY no definido: la búsqueda semántica y las ayudas con IA quedan apagadas");
    }

    let index = Arc::new(index::VecIndex::default());
    index.load(&pool).await.expect("no se pudo cargar el índice semántico");

    let state = AppState { pool, keys: Arc::new(auth::Keys::new(secret.as_bytes())), ai, index };

    let per_min = |n| Arc::new(guard::Limiter::new(n, std::time::Duration::from_secs(60)));
    // contraseñas: pocas por minuto y por IP; IA: cuesta dinero en cada llamada
    let auth_limit = middleware::from_fn_with_state(per_min(30), guard::limit);
    let ai_limit = middleware::from_fn_with_state(per_min(30), guard::limit);

    let auth_routes = Router::new()
        .route("/api/users/register", post(users::register))
        .route("/api/users/login", post(users::login))
        .layer(auth_limit);
    let ai_routes = Router::new()
        .route("/api/ai/search", post(ai::search))
        .route("/api/ai/recommended", get(ai::recommended))
        .route("/api/ai/improve-job", post(ai::improve_job))
        .route("/api/ai/cover-letter", post(ai::cover_letter))
        .layer(ai_limit);

    let mut app = Router::new()
        .route("/api/health", get(health))
        .merge(auth_routes)
        .merge(ai_routes)
        .route("/api/users/me", get(users::me).put(users::update_me))
        // vacantes (las rutas fijas ganan a «{id}»)
        .route("/api/jobs", get(jobs::list).post(jobs::create))
        .route("/api/jobs/my-jobs", get(jobs::my_jobs))
        .route("/api/jobs/my-applications", get(jobs::my_applications))
        .route("/api/jobs/{id}", get(jobs::get_one).put(jobs::update).delete(jobs::remove))
        .route("/api/jobs/{id}/apply", post(jobs::apply))
        .route("/api/jobs/{id}/applicants", get(jobs::applicants))
        .route("/api/jobs/{id}/applicants/{app_id}", put(jobs::set_status))
        .route("/api/jobs/{id}/match", get(ai::match_score))
        // notificaciones
        .route("/api/notifications", get(notifications::list))
        .route("/api/notifications/mark-all", put(notifications::mark_all))
        .route("/api/notifications/{id}/read", put(notifications::mark_read))
        // una ruta /api que no existe responde JSON, no la página del sitio
        .route("/api/{*rest}", any(|| async { error::AppError::NotFound }))
        .with_state(state);

    // El sitio (Vue compilado) lo sirve el mismo proceso: una sola dirección, sin CORS en producción.
    if let Ok(dir) = std::env::var("STATIC_DIR") {
        let index = ServeFile::new(format!("{dir}/index.html"));
        app = app.fallback_service(ServeDir::new(&dir).fallback(index));
        tracing::info!("sirviendo el sitio desde {dir}");
    }
    let app = app
        .layer(CompressionLayer::new())
        .layer(match std::env::var("CORS_ORIGIN") {
            Ok(o) if !o.is_empty() => CorsLayer::new()
                .allow_origin(o.parse::<axum::http::HeaderValue>().expect("CORS_ORIGIN inválido"))
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
            _ => CorsLayer::very_permissive(),
        })
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(guard::headers));

    let port: u16 = env("PORT", "4000").parse().expect("PORT inválido");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("no se pudo abrir el puerto");
    tracing::info!("escuchando en http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .unwrap();
}

async fn health(axum::extract::State(s): axum::extract::State<AppState>) -> Result<&'static str, error::AppError> {
    s.pool.get().await?.simple_query("SELECT 1").await?;
    Ok("ok")
}
