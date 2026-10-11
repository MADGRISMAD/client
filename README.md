# IAplica — bolsa de trabajo para estudiantes

Frontend en **Vue 3 + Vite + Tailwind** y backend en **Rust (Axum + Tokio) + PostgreSQL**, con IA (Gemini) para
buscar y recomendar empleos por significado.

```
src/      frontend (Vue)
server/   API en Rust
```

## Arrancar

```bash
# 1. Base de datos (Postgres 14+, sin extensiones que instalar: usa pg_trgm y unaccent, que ya vienen)
createdb empleos

# 2. API (puerto 4000)
cd server
cp .env.example .env          # ajusta DATABASE_URL y JWT_SECRET
export $(grep -v '^#' .env | xargs)
cargo run --release           # aplica las migraciones al arrancar

# 3. Frontend (otra terminal, desde la raíz)
npm install && npm run dev    # http://localhost:5173
```

El frontend lee la API de `VITE_API_URL` (por defecto `http://localhost:4000`).
> En macOS el puerto 5000 lo usa AirPlay, por eso la API usa el 4000.

## Diseño

Una sola paleta (neutros fríos + esmeralda como único acento), tipografía Geist (propia, sin Google Fonts), iconos
Phosphor y una sola escala de radios (12 px; solo las etiquetas son píldora). Claro y oscuro según el sistema.
Las escalas `gray` y `emerald` de Tailwind se redefinen en `src/assets/tailwind.css`, así que todas las pantallas
cambian juntas y el modo oscuro no necesita `dark:` en cada elemento. El movimiento respeta
`prefers-reduced-motion` (entrada de la portada, `v-reveal` al entrar en pantalla, retroalimentación al presionar).
Las fotos de `public/img` son de relleno (Picsum): reemplázalas por fotografía real de estudiantes y trabajo.

## API

| Ruta | Quién | Qué hace |
|---|---|---|
| `POST /api/users/register`, `/login` · `GET/PUT /api/users/me` | todos | Cuenta. Estudiantes con correo `.edu` |
| `GET /api/jobs?q=&remote=&minSalary=&limit=&page=` | público | Lista y busca (sin acentos, tolera errores de dedo) |
| `GET /api/jobs/{id}` | público | Detalle |
| `POST /api/jobs` · `PUT/DELETE /api/jobs/{id}` · `GET /api/jobs/my-jobs` | empleador | Publicar y administrar |
| `POST /api/jobs/{id}/apply` · `GET /api/jobs/my-applications` | estudiante | Postularse y ver mis postulaciones |
| `GET /api/jobs/{id}/applicants` · `PUT /api/jobs/{id}/applicants/{app}` | empleador | Ver postulantes y cambiar su estado |
| `GET /api/notifications` · `PUT …/{id}/read` · `PUT …/mark-all` | sesión | Avisos |
| `POST /api/ai/search` | sesión | Búsqueda por significado, con `matchScore` 0–100 |
| `GET /api/ai/recommended` | estudiante | Vacantes que más encajan con su perfil |
| `GET /api/jobs/{id}/match` | estudiante | Qué tanto encaja con la vacante, habilidades que coinciden y que faltan |
| `POST /api/ai/improve-job` | empleador | Pule la descripción y sugiere etiquetas, requisitos y beneficios |
| `POST /api/ai/cover-letter` | estudiante | Borrador de carta de presentación |

Los errores siempre salen como `{ "message": "…" }`.

## Por qué es rápido

- **Sin ORM**: una consulta preparada por endpoint, filas a JSON tipado, pool de conexiones, mimalloc y LTO.
- **Búsqueda de texto** en dos pasos: primero texto completo en español sin acentos (índice GIN); solo si no alcanza,
  se completa con parecidos por trigramas (errores de dedo).
- **Búsqueda con IA sin pgvector**: los vectores de las vacantes (256 dimensiones, normalizados) viven en un arreglo
  plano en RAM; buscar es un producto punto contra todos. La vacante se vectoriza en segundo plano al crearla o editarla.
- Los vectores de las consultas ya hechas se guardan en memoria: repetir una búsqueda no llama a Gemini.

Medido en un MacBook (API, Postgres y el cliente de pruebas en la misma máquina), con **50 mil vacantes**:

| Prueba | Resultado |
|---|---|
| Lista de 50 vacantes | ≈ 17 400 peticiones/s, p99 5 ms |
| Detalle por id | ≈ 20 000 peticiones/s, p99 4 ms |
| Búsqueda de texto | ≈ 930 peticiones/s, mediana 25 ms |
| Búsqueda con error de dedo | ≈ 220 peticiones/s |
| Búsqueda con IA (50 mil vectores, con Gemini simulado) | ≈ 980 peticiones/s, mediana 31 ms |

Con Gemini real la búsqueda con IA suma el viaje de red de la consulta (la primera vez; después sale de memoria).

## IA (opcional)

Sin `GEMINI_API_KEY` todo funciona menos `/api/ai/*` (responden 503 con un mensaje claro). Variables en
`server/.env.example`. ponytail: el índice vive en el proceso; con varias instancias, cada una carga el suyo al
arrancar. Para escalar a varias, pasar a pgvector (HNSW).

## Producción (VPS)

Cada `push` a `main` corre `.github/workflows/deploy.yml`: revisa y prueba la API, compila el sitio, construye **una sola
imagen** (sitio + API, ~100 MB) en GitHub y la sube al VPS; ahí `docker compose` la levanta junto a Postgres.
El sitio y la API salen por la misma dirección (`/api/…`), así que no hay CORS en producción.

- Carpeta en el servidor: `~/iaplica` (`docker-compose.yml` y `.env`, que **solo vive allá**: contraseña de Postgres,
  `JWT_SECRET` y `GEMINI_API_KEY`).
- Caddy hace el HTTPS y manda todo al puerto `127.0.0.1:8084`.
- Respaldo diario de la base con el resto de los proyectos (`respaldar-bases`, últimos 7 días).
- Para activar la IA: poner `GEMINI_API_KEY` en `~/iaplica/.env` y `docker compose up -d`.
- Contraseñas y gasto de IA están protegidos con límite de intentos por IP, y la respuesta lleva cabeceras de seguridad.

### Dominio `iaplica.mx`
1. Registrar el dominio y crear un registro **A** `iaplica.mx` → `15.235.62.27` (y otro para `www`).
2. En el VPS, en `/etc/caddy/Caddyfile`, cambiar `iaplica.15-235-62-27.sslip.io` por `iaplica.mx, www.iaplica.mx`
   y poner `CORS_ORIGIN=https://iaplica.mx` en `.env`; `sudo systemctl reload caddy`.

## Pruebas

```bash
cd server
cargo test                                  # índice semántico
# con el servidor en marcha y la base vacía:
python3 scripts/smoke.py                    # comprobaciones de la API de punta a punta
python3 scripts/fake_gemini.py &            # Gemini de mentira
GEMINI_API_KEY=fake-key GEMINI_BASE=http://127.0.0.1:4999 cargo run --release   # y en otra terminal:
python3 scripts/smoke_ai.py                 # IA: búsqueda, recomendaciones, match, asistentes
```
