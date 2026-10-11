use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{extract::J, auth::{self, Auth}, error::AppError, AppState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOut {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub role: String,
    pub full_name: String,
    pub email: String,
    pub university: String,
    pub degree: String,
    pub cv_url: String,
    pub skills: Vec<String>,
    pub company: Value,
}

pub const USER_COLS: &str = "id, role, full_name, email, university, degree, cv_url, skills, company";

pub fn user_from_row(r: &tokio_postgres::Row) -> UserOut {
    UserOut {
        id: r.get(0),
        role: r.get(1),
        full_name: r.get(2),
        email: r.get(3),
        university: r.get(4),
        degree: r.get(5),
        cv_url: r.get(6),
        skills: r.get(7),
        company: r.get(8),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    role: String,
    full_name: String,
    email: String,
    password: String,
    #[serde(default)]
    university: String,
    #[serde(default)]
    degree: String,
    #[serde(default)]
    skills: Vec<String>,
    #[serde(default)]
    company: Value,
}

/// «correo.edu», «correo.edu.mx», «correo.edu.co»…
fn is_edu(email: &str) -> bool {
    let host = email.rsplit('@').next().unwrap_or("");
    host.ends_with(".edu") || host.contains(".edu.")
}

fn clean_skills(v: Vec<String>) -> Vec<String> {
    v.into_iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty() && s.len() <= 60).take(40).collect()
}

pub async fn register(State(s): State<AppState>, J(b): J<Register>) -> Result<(StatusCode, Json<Value>), AppError> {
    let email = b.email.trim().to_lowercase();
    let name = b.full_name.trim().to_string();
    if b.role != "student" && b.role != "employer" {
        return Err(AppError::BadRequest("Elige si eres estudiante o empleador.".into()));
    }
    if name.is_empty() || !email.contains('@') || b.password.len() < 8 {
        return Err(AppError::BadRequest("Revisa tu nombre, tu correo y que la contraseña tenga al menos 8 caracteres.".into()));
    }
    if b.role == "student" && !is_edu(&email) {
        return Err(AppError::BadRequest("El correo debe terminar en .edu".into()));
    }
    let company = if b.role == "employer" { b.company } else { json!({}) };
    let hash = auth::hash(b.password).await?;
    let id = Uuid::now_v7();
    let c = s.pool.get().await?;
    let skills = clean_skills(b.skills);
    c.execute(
        "INSERT INTO users (id, role, full_name, email, password, university, degree, skills, company) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
        &[&id, &b.role, &name, &email, &hash, &b.university.trim(), &b.degree.trim(), &skills, &company],
    )
    .await
    .map_err(|e| match AppError::from(e) {
        AppError::Conflict(_) => AppError::Conflict("Ese correo ya está registrado.".into()),
        other => other,
    })?;
    if b.role == "student" {
        crate::ai::refresh_candidate(s, id);
    }
    Ok((StatusCode::CREATED, Json(json!({ "message": "Registro exitoso. Ya puedes iniciar sesión." }))))
}

#[derive(Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

pub async fn login(State(s): State<AppState>, J(b): J<Login>) -> Result<Json<Value>, AppError> {
    let c = s.pool.get().await?;
    let row = c
        .query_opt(&format!("SELECT {USER_COLS}, password FROM users WHERE lower(email) = $1"), &[&b.email.trim().to_lowercase()])
        .await?;
    // misma respuesta con correo inexistente o contraseña mala: no se revela cuál falló
    let bad = || AppError::BadRequest("Correo o contraseña incorrectos.".into());
    let row = row.ok_or_else(bad)?;
    let stored: String = row.get(9);
    if !auth::verify(b.password, stored).await {
        return Err(bad());
    }
    let user = user_from_row(&row);
    let token = auth::token(&s.keys, user.id, &user.role)?;
    Ok(Json(json!({ "token": token, "user": user })))
}

pub async fn me(State(s): State<AppState>, a: Auth) -> Result<Json<UserOut>, AppError> {
    let c = s.pool.get().await?;
    let row = c.query_opt(&format!("SELECT {USER_COLS} FROM users WHERE id = $1"), &[&a.id]).await?.ok_or(AppError::NotFound)?;
    Ok(Json(user_from_row(&row)))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMe {
    full_name: Option<String>,
    university: Option<String>,
    degree: Option<String>,
    cv_url: Option<String>,
    skills: Option<Vec<String>>,
    company: Option<Value>,
}

pub async fn update_me(State(s): State<AppState>, a: Auth, J(b): J<UpdateMe>) -> Result<Json<UserOut>, AppError> {
    let skills = b.skills.map(clean_skills);
    let c = s.pool.get().await?;
    let row = c
        .query_one(
            &format!(
                "UPDATE users SET full_name = coalesce($2, full_name), university = coalesce($3, university), degree = coalesce($4, degree),
                        cv_url = coalesce($5, cv_url), skills = coalesce($6, skills), company = coalesce($7, company)
                 WHERE id = $1 RETURNING {USER_COLS}"
            ),
            &[&a.id, &b.full_name.map(|x| x.trim().to_string()).filter(|x| !x.is_empty()), &b.university, &b.degree, &b.cv_url, &skills, &b.company],
        )
        .await?;
    if a.role == "student" {
        crate::ai::refresh_candidate(s, a.id);
    }
    Ok(Json(user_from_row(&row)))
}
