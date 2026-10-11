use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{extract::{J, P, Q}, auth::Auth, error::AppError, notifications::notify, AppState};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Salary {
    pub min: Option<i32>,
    pub max: Option<i32>,
    #[serde(rename = "type", default = "hora")]
    pub kind: String,
    #[serde(default = "usd")]
    pub currency: String,
}
fn hora() -> String { "hora".into() }
fn usd() -> String { "USD".into() }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobOut {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub created_by: Uuid,
    pub title: String,
    pub description: String,
    pub company: String,
    pub tags: Vec<String>,
    pub is_remote: bool,
    pub salary_range: Salary,
    pub duration: String,
    pub highlighted: bool,
    pub requirements: Vec<String>,
    pub responsibilities: Vec<String>,
    pub benefits: Vec<String>,
    pub created_at: DateTime<Utc>,
    /// Solo en la búsqueda con IA y en las recomendaciones: 0–100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_score: Option<u8>,
}

pub const JOB_COLS: &str = "j.id, j.created_by, j.title, j.description, j.company, j.tags, j.is_remote, j.salary_min, j.salary_max, j.salary_type, j.currency, \
     j.duration, j.highlighted, j.requirements, j.responsibilities, j.benefits, j.created_at";

pub fn job_from_row(r: &tokio_postgres::Row) -> JobOut {
    JobOut {
        id: r.get(0),
        created_by: r.get(1),
        title: r.get(2),
        description: r.get(3),
        company: r.get(4),
        tags: r.get(5),
        is_remote: r.get(6),
        salary_range: Salary { min: r.get(7), max: r.get(8), kind: r.get(9), currency: r.get(10) },
        duration: r.get(11),
        highlighted: r.get(12),
        requirements: r.get(13),
        responsibilities: r.get(14),
        benefits: r.get(15),
        created_at: r.get(16),
        match_score: None,
    }
}

#[derive(Deserialize)]
pub struct ListQuery {
    q: Option<String>,
    remote: Option<bool>,
    #[serde(rename = "minSalary")]
    min_salary: Option<i32>,
    limit: Option<i64>,
    page: Option<i64>,
}

/// Lista y búsqueda. Sin `q`: las más recientes (destacadas primero). Con `q`, en dos pasos para que lo común sea
/// rapidísimo: 1) texto completo en español sin acentos (GIN) ordenado por relevancia; 2) solo si no alcanzan
/// resultados, se completa con parecidos por trigramas (tolera errores de dedo: «desarolador»), que cuesta más.
pub async fn list(State(s): State<AppState>, Q(q): Q<ListQuery>) -> Result<Json<Vec<JobOut>>, AppError> {
    let text = q.q.as_deref().map(str::trim).filter(|t| !t.is_empty()).map(|t| t.chars().take(120).collect::<String>());
    let limit = q.limit.unwrap_or(50).clamp(1, 100);
    let offset = (q.page.unwrap_or(1).max(1) - 1) * limit;
    let c = s.pool.get().await?;

    let Some(t) = text else {
        let st = c
            .prepare_cached(&format!(
                "SELECT {JOB_COLS} FROM jobs j WHERE ($1::bool IS NULL OR j.is_remote = $1) AND ($2::int IS NULL OR j.salary_max >= $2)
                 ORDER BY j.highlighted DESC, j.created_at DESC, j.id DESC LIMIT $3 OFFSET $4"
            ))
            .await?;
        let rows = c.query(&st, &[&q.remote, &q.min_salary, &limit, &offset]).await?;
        return Ok(Json(rows.iter().map(job_from_row).collect()));
    };

    let st = c
        .prepare_cached(&format!(
            "SELECT {JOB_COLS} FROM jobs j
             WHERE j.search @@ websearch_to_tsquery('spanish', f_unaccent($1))
               AND ($2::bool IS NULL OR j.is_remote = $2) AND ($3::int IS NULL OR j.salary_max >= $3)
             ORDER BY j.highlighted DESC, ts_rank(j.search, websearch_to_tsquery('spanish', f_unaccent($1))) DESC, j.created_at DESC, j.id DESC
             LIMIT $4 OFFSET $5"
        ))
        .await?;
    let mut out: Vec<JobOut> = c.query(&st, &[&t, &q.remote, &q.min_salary, &limit, &offset]).await?.iter().map(job_from_row).collect();

    if (out.len() as i64) < limit && offset == 0 {
        let have: Vec<Uuid> = out.iter().map(|j| j.id).collect();
        let st = c
            .prepare_cached(&format!(
                "SELECT {JOB_COLS} FROM jobs j
                 WHERE f_unaccent(j.title) % f_unaccent($1) AND j.id <> ALL($5)
                   AND ($2::bool IS NULL OR j.is_remote = $2) AND ($3::int IS NULL OR j.salary_max >= $3)
                 ORDER BY similarity(f_unaccent(j.title), f_unaccent($1)) DESC, j.highlighted DESC, j.id DESC LIMIT $4"
            ))
            .await?;
        let more = c.query(&st, &[&t, &q.remote, &q.min_salary, &(limit - out.len() as i64), &have]).await?;
        out.extend(more.iter().map(job_from_row));
    }
    Ok(Json(out))
}

pub async fn get_one(State(s): State<AppState>, P(id): P<Uuid>) -> Result<Json<JobOut>, AppError> {
    let c = s.pool.get().await?;
    let st = c.prepare_cached(&format!("SELECT {JOB_COLS} FROM jobs j WHERE j.id = $1")).await?;
    let row = c.query_opt(&st, &[&id]).await?.ok_or(AppError::NotFound)?;
    Ok(Json(job_from_row(&row)))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobIn {
    title: String,
    description: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    is_remote: bool,
    #[serde(default)]
    salary_range: Salary,
    #[serde(default)]
    duration: String,
    #[serde(default)]
    highlighted: bool,
    #[serde(default)]
    requirements: Vec<String>,
    #[serde(default)]
    responsibilities: Vec<String>,
    #[serde(default)]
    benefits: Vec<String>,
}

fn list_clean(v: &[String], max: usize) -> Vec<String> {
    v.iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty() && s.len() <= 200).take(max).collect()
}

fn validate(b: &JobIn) -> Result<(), AppError> {
    if b.title.trim().is_empty() || b.title.len() > 150 || b.description.trim().is_empty() || b.description.len() > 8000 {
        return Err(AppError::BadRequest("El título y la descripción son obligatorios (título hasta 150 letras).".into()));
    }
    if let (Some(a), Some(z)) = (b.salary_range.min, b.salary_range.max) {
        if a > z {
            return Err(AppError::BadRequest("El salario mínimo no puede ser mayor que el máximo.".into()));
        }
    }
    Ok(())
}

pub async fn create(State(s): State<AppState>, a: Auth, J(b): J<JobIn>) -> Result<(StatusCode, Json<JobOut>), AppError> {
    a.employer()?;
    validate(&b)?;
    let id = Uuid::now_v7();
    let c = s.pool.get().await?;
    let row = c
        .query_one(
            &format!(
                "WITH me AS (SELECT coalesce(nullif(company->>'name', ''), full_name) AS name FROM users WHERE id = $2)
                 INSERT INTO jobs AS j (id, created_by, title, description, company, tags, is_remote, salary_min, salary_max, salary_type, currency, duration, highlighted, requirements, responsibilities, benefits)
                 SELECT $1::uuid, $2::uuid, $3::text, $4::text, me.name, $5::text[], $6::bool, $7::int, $8::int, $9::text, $10::text, $11::text, $12::bool, $13::text[], $14::text[], $15::text[] FROM me
                 RETURNING {JOB_COLS}"
            ),
            &[
                &id, &a.id, &b.title.trim(), &b.description.trim(), &list_clean(&b.tags, 20), &b.is_remote, &b.salary_range.min, &b.salary_range.max,
                &b.salary_range.kind, &b.salary_range.currency, &b.duration.trim(), &b.highlighted, &list_clean(&b.requirements, 30),
                &list_clean(&b.responsibilities, 30), &list_clean(&b.benefits, 30),
            ],
        )
        .await?;
    let out = job_from_row(&row);
    crate::ai::refresh_job(s, id);
    Ok((StatusCode::CREATED, Json(out)))
}

pub async fn my_jobs(State(s): State<AppState>, a: Auth) -> Result<Json<Vec<Value>>, AppError> {
    a.employer()?;
    let c = s.pool.get().await?;
    let st = c
        .prepare_cached(&format!(
            "SELECT {JOB_COLS}, (SELECT count(*) FROM applications x WHERE x.job_id = j.id) FROM jobs j WHERE j.created_by = $1 ORDER BY j.created_at DESC"
        ))
        .await?;
    let rows = c.query(&st, &[&a.id]).await?;
    Ok(Json(
        rows.iter()
            .map(|r| {
                let mut v = serde_json::to_value(job_from_row(r)).unwrap_or_default();
                v["applicantsCount"] = json!(r.get::<_, i64>(17));
                v
            })
            .collect(),
    ))
}

pub async fn update(State(s): State<AppState>, a: Auth, P(id): P<Uuid>, J(b): J<JobIn>) -> Result<Json<JobOut>, AppError> {
    a.employer()?;
    validate(&b)?;
    let c = s.pool.get().await?;
    let row = c
        .query_opt(
            &format!(
                "UPDATE jobs j SET title=$3, description=$4, tags=$5, is_remote=$6, salary_min=$7, salary_max=$8, salary_type=$9, currency=$10, duration=$11,
                        highlighted=$12, requirements=$13, responsibilities=$14, benefits=$15
                 WHERE j.id = $1 AND j.created_by = $2 RETURNING {JOB_COLS}"
            ),
            &[
                &id, &a.id, &b.title.trim(), &b.description.trim(), &list_clean(&b.tags, 20), &b.is_remote, &b.salary_range.min, &b.salary_range.max,
                &b.salary_range.kind, &b.salary_range.currency, &b.duration.trim(), &b.highlighted, &list_clean(&b.requirements, 30),
                &list_clean(&b.responsibilities, 30), &list_clean(&b.benefits, 30),
            ],
        )
        .await?
        .ok_or(AppError::NotFound)?;
    crate::ai::refresh_job(s, id);
    Ok(Json(job_from_row(&row)))
}

pub async fn remove(State(s): State<AppState>, a: Auth, P(id): P<Uuid>) -> Result<StatusCode, AppError> {
    a.employer()?;
    let n = s.pool.get().await?.execute("DELETE FROM jobs WHERE id = $1 AND created_by = $2", &[&id, &a.id]).await?;
    if n == 0 {
        return Err(AppError::NotFound);
    }
    s.index.remove(id);
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyIn {
    #[serde(default)]
    cover_letter: String,
}

pub async fn apply(State(s): State<AppState>, a: Auth, P(id): P<Uuid>, J(b): J<ApplyIn>) -> Result<(StatusCode, Json<Value>), AppError> {
    a.student()?;
    if b.cover_letter.len() > 4000 {
        return Err(AppError::BadRequest("La carta de presentación es demasiado larga.".into()));
    }
    let c = s.pool.get().await?;
    let job = c.query_opt("SELECT created_by, title FROM jobs WHERE id = $1", &[&id]).await?.ok_or(AppError::NotFound)?;
    let (owner, title): (Uuid, String) = (job.get(0), job.get(1));
    c.execute("INSERT INTO applications (id, job_id, user_id, cover_letter) VALUES ($1,$2,$3,$4)", &[&Uuid::now_v7(), &id, &a.id, &b.cover_letter.trim()])
        .await
        .map_err(|e| match AppError::from(e) {
            AppError::Conflict(_) => AppError::Conflict("Ya te postulaste a esta vacante.".into()),
            other => other,
        })?;
    let name: String = c.query_one("SELECT full_name FROM users WHERE id = $1", &[&a.id]).await?.get(0);
    drop(c);
    notify(&s.pool, owner, &format!("{name} se postuló a «{title}»"), &format!("/jobs/{id}/applicants")).await;
    Ok((StatusCode::CREATED, Json(json!({ "message": "Postulación enviada." }))))
}

pub async fn applicants(State(s): State<AppState>, a: Auth, P(id): P<Uuid>) -> Result<Json<Vec<Value>>, AppError> {
    a.employer()?;
    let c = s.pool.get().await?;
    if c.query_opt("SELECT 1 FROM jobs WHERE id = $1 AND created_by = $2", &[&id, &a.id]).await?.is_none() {
        return Err(AppError::NotFound);
    }
    let rows = c
        .query(
            "SELECT p.id, p.cover_letter, p.status, p.applied_at, u.id, u.full_name, u.email, u.university, u.degree, u.skills, u.cv_url
             FROM applications p JOIN users u ON u.id = p.user_id WHERE p.job_id = $1 ORDER BY p.applied_at DESC",
            &[&id],
        )
        .await?;
    Ok(Json(
        rows.iter()
            .map(|r| {
                json!({
                    "_id": r.get::<_, Uuid>(0), "coverLetter": r.get::<_, String>(1), "status": r.get::<_, String>(2), "appliedAt": r.get::<_, DateTime<Utc>>(3),
                    "user": { "_id": r.get::<_, Uuid>(4), "fullName": r.get::<_, String>(5), "email": r.get::<_, String>(6), "university": r.get::<_, String>(7),
                              "degree": r.get::<_, String>(8), "skills": r.get::<_, Vec<String>>(9), "cvUrl": r.get::<_, String>(10) }
                })
            })
            .collect(),
    ))
}

#[derive(Deserialize)]
pub struct StatusIn {
    status: String,
}

pub async fn set_status(State(s): State<AppState>, a: Auth, P((id, app_id)): P<(Uuid, Uuid)>, J(b): J<StatusIn>) -> Result<Json<Value>, AppError> {
    a.employer()?;
    if !["applied", "viewed", "interview", "hired", "rejected"].contains(&b.status.as_str()) {
        return Err(AppError::BadRequest("Estado no válido.".into()));
    }
    let c = s.pool.get().await?;
    let row = c
        .query_opt(
            "UPDATE applications p SET status = $4 FROM jobs j WHERE p.id = $3 AND p.job_id = $1 AND j.id = p.job_id AND j.created_by = $2 RETURNING p.user_id, j.title",
            &[&id, &a.id, &app_id, &b.status],
        )
        .await?
        .ok_or(AppError::NotFound)?;
    let (student, title): (Uuid, String) = (row.get(0), row.get(1));
    drop(c);
    let label = match b.status.as_str() {
        "viewed" => "fue revisada",
        "interview" => "avanzó a entrevista",
        "hired" => "fue aceptada: ¡te contrataron!",
        "rejected" => "no fue seleccionada",
        _ => "volvió a estado aplicado",
    };
    notify(&s.pool, student, &format!("Tu postulación a «{title}» {label}"), "/my-applications").await;
    Ok(Json(json!({ "status": b.status })))
}

pub async fn my_applications(State(s): State<AppState>, a: Auth) -> Result<Json<Vec<Value>>, AppError> {
    a.student()?;
    let c = s.pool.get().await?;
    let st = c
        .prepare_cached(&format!(
            "SELECT {JOB_COLS}, p.status, p.applied_at FROM applications p JOIN jobs j ON j.id = p.job_id WHERE p.user_id = $1 ORDER BY p.applied_at DESC"
        ))
        .await?;
    let rows = c.query(&st, &[&a.id]).await?;
    Ok(Json(
        rows.iter()
            .map(|r| {
                let mut v = serde_json::to_value(job_from_row(r)).unwrap_or_default();
                v["status"] = json!(r.get::<_, String>(17));
                v["appliedAt"] = json!(r.get::<_, DateTime<Utc>>(18));
                v
            })
            .collect(),
    ))
}

