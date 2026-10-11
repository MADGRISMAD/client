use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

/// Pool con el doble de conexiones que núcleos (mínimo 8): las consultas son cortas y no bloquean el hilo.
pub async fn connect(url: &str) -> Result<Pool, Box<dyn std::error::Error>> {
    let cfg: tokio_postgres::Config = url.parse()?;
    let mgr = Manager::from_config(cfg, NoTls, ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let size = (std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4) * 2).max(8);
    let pool = Pool::builder(mgr).max_size(size).build()?;
    let _ = pool.get().await?; // falla pronto si la base no responde
    Ok(pool)
}

const MIGRATIONS: &[(&str, &str)] = &[
    ("001_init", include_str!("../migrations/001_init.sql")),
    ("002_recent_index", include_str!("../migrations/002_recent_index.sql")),
];

/// Aplica las migraciones pendientes, cada una en su transacción. ponytail: sin down-migrations ni checksums.
pub async fn migrate(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let mut c = pool.get().await?;
    c.batch_execute("CREATE TABLE IF NOT EXISTS _migrations (name text PRIMARY KEY, applied_at timestamptz NOT NULL DEFAULT now())").await?;
    for (name, sql) in MIGRATIONS {
        if c.query_opt("SELECT 1 FROM _migrations WHERE name = $1", &[name]).await?.is_some() {
            continue;
        }
        let tx = c.transaction().await?;
        tx.batch_execute(sql).await?;
        tx.execute("INSERT INTO _migrations (name) VALUES ($1)", &[name]).await?;
        tx.commit().await?;
        tracing::info!("migración aplicada: {name}");
    }
    Ok(())
}
