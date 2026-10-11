-- Esquema inicial. Extensiones que vienen con cualquier Postgres (sin instalar nada extra).
CREATE EXTENSION IF NOT EXISTS pg_trgm;   -- búsqueda tolerante a errores de dedo
CREATE EXTENSION IF NOT EXISTS unaccent;  -- «analista» = «Analista», «diseño» = «diseno»

-- unaccent no es IMMUTABLE; una envoltura sí lo es y permite usarla en columnas generadas e índices.
CREATE OR REPLACE FUNCTION f_unaccent(text) RETURNS text
    LANGUAGE sql IMMUTABLE PARALLEL SAFE STRICT AS $$ SELECT public.unaccent('public.unaccent', $1) $$;

-- array_to_string es STABLE; para usarla en una columna generada se envuelve (con text[] el resultado no cambia).
CREATE OR REPLACE FUNCTION f_join(text[]) RETURNS text
    LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $$ SELECT array_to_string($1, ' ') $$;

CREATE TABLE users (
    id          uuid PRIMARY KEY,
    role        text NOT NULL CHECK (role IN ('student', 'employer')),
    full_name   text NOT NULL,
    email       text NOT NULL,
    password    text NOT NULL,
    university  text NOT NULL DEFAULT '',
    degree      text NOT NULL DEFAULT '',
    cv_url      text NOT NULL DEFAULT '',
    skills      text[] NOT NULL DEFAULT '{}',
    company     jsonb NOT NULL DEFAULT '{}',          -- {name, website, description}
    embedding   real[],                                 -- perfil del candidato para el «match» con IA
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX users_email_key ON users (lower(email));

CREATE TABLE jobs (
    id            uuid PRIMARY KEY,
    created_by    uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    title         text NOT NULL,
    description   text NOT NULL,
    company       text NOT NULL DEFAULT '',
    tags          text[] NOT NULL DEFAULT '{}',
    is_remote     boolean NOT NULL DEFAULT false,
    salary_min    integer,
    salary_max    integer,
    salary_type   text NOT NULL DEFAULT 'hora',
    currency      text NOT NULL DEFAULT 'USD',
    duration      text NOT NULL DEFAULT '',
    highlighted   boolean NOT NULL DEFAULT false,
    requirements     text[] NOT NULL DEFAULT '{}',
    responsibilities text[] NOT NULL DEFAULT '{}',
    benefits         text[] NOT NULL DEFAULT '{}',
    embedding     real[],                               -- vector semántico de la vacante (IA)
    created_at    timestamptz NOT NULL DEFAULT now(),
    -- texto de búsqueda ya normalizado: sin acentos, en español, título pesa más que la descripción
    search        tsvector GENERATED ALWAYS AS (
        setweight(to_tsvector('spanish', f_unaccent(title)), 'A') ||
        setweight(to_tsvector('spanish', f_unaccent(coalesce(company, '') || ' ' || f_join(tags))), 'B') ||
        setweight(to_tsvector('spanish', f_unaccent(description)), 'C')
    ) STORED
);
CREATE INDEX jobs_search_idx   ON jobs USING gin (search);
CREATE INDEX jobs_title_trgm   ON jobs USING gin (f_unaccent(title) gin_trgm_ops);
CREATE INDEX jobs_tags_idx     ON jobs USING gin (tags);
CREATE INDEX jobs_recent_idx   ON jobs (highlighted DESC, created_at DESC);
CREATE INDEX jobs_owner_idx    ON jobs (created_by, created_at DESC);

CREATE TABLE applications (
    id           uuid PRIMARY KEY,
    job_id       uuid NOT NULL REFERENCES jobs (id) ON DELETE CASCADE,
    user_id      uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    cover_letter text NOT NULL DEFAULT '',
    status       text NOT NULL DEFAULT 'applied' CHECK (status IN ('applied', 'viewed', 'interview', 'hired', 'rejected')),
    applied_at   timestamptz NOT NULL DEFAULT now(),
    UNIQUE (job_id, user_id)
);
CREATE INDEX applications_user_idx ON applications (user_id, applied_at DESC);

CREATE TABLE notifications (
    id         uuid PRIMARY KEY,
    user_id    uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    message    text NOT NULL,
    link       text NOT NULL DEFAULT '',
    read       boolean NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX notifications_user_idx ON notifications (user_id, created_at DESC);
