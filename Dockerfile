# IAplica: el sitio (Vue) y la API (Rust) en una sola imagen pequeña. Se construye en GitHub Actions, no en el VPS.

# ---- sitio
FROM node:22-alpine AS web
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY index.html vite.config.js ./
COPY public ./public
COPY src ./src
# En producción la API está en la misma dirección que el sitio (rutas relativas /api/…)
ENV VITE_API_URL=""
RUN npm run build

# ---- API
FROM rust:1-bookworm AS api
WORKDIR /app
# primero solo las dependencias: se reutiliza la capa mientras no cambie Cargo.toml
COPY server/Cargo.toml server/Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src
COPY server/migrations ./migrations
COPY server/src ./src
RUN touch src/main.rs && cargo build --release

# ---- imagen final
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* && useradd --system --uid 10001 app
COPY --from=api /app/target/release/server /usr/local/bin/server
COPY --from=web /app/dist /srv/www
ENV STATIC_DIR=/srv/www PORT=8080
USER app
EXPOSE 8080
HEALTHCHECK --interval=15s --timeout=3s --retries=5 CMD curl -fs http://127.0.0.1:8080/api/health || exit 1
CMD ["server"]
