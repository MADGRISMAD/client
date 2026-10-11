use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{auth::Auth, extract::P, error::AppError, AppState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationOut {
    #[serde(rename = "_id")]
    id: Uuid,
    message: String,
    link: String,
    read: bool,
    created_at: DateTime<Utc>,
}

pub async fn list(State(s): State<AppState>, a: Auth) -> Result<Json<Vec<NotificationOut>>, AppError> {
    let c = s.pool.get().await?;
    let st = c.prepare_cached("SELECT id, message, link, read, created_at FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50").await?;
    let rows = c.query(&st, &[&a.id]).await?;
    Ok(Json(rows.iter().map(|r| NotificationOut { id: r.get(0), message: r.get(1), link: r.get(2), read: r.get(3), created_at: r.get(4) }).collect()))
}

pub async fn mark_read(State(s): State<AppState>, a: Auth, P(id): P<Uuid>) -> Result<StatusCode, AppError> {
    s.pool.get().await?.execute("UPDATE notifications SET read = true WHERE id = $1 AND user_id = $2", &[&id, &a.id]).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn mark_all(State(s): State<AppState>, a: Auth) -> Result<StatusCode, AppError> {
    s.pool.get().await?.execute("UPDATE notifications SET read = true WHERE user_id = $1 AND NOT read", &[&a.id]).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Crea un aviso; si falla no debe tumbar la acción principal.
pub async fn notify(pool: &deadpool_postgres::Pool, user: Uuid, message: &str, link: &str) {
    if let Ok(c) = pool.get().await {
        let _ = c.execute("INSERT INTO notifications (id, user_id, message, link) VALUES ($1,$2,$3,$4)", &[&Uuid::now_v7(), &user, &message, &link]).await;
    }
}
