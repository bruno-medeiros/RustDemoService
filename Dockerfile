# syntax=docker/dockerfile:1.7
# Requires BuildKit (DOCKER_BUILDKIT=1 or Docker >= 23 which enables it by default).
# The 1.7 frontend unlocks --mount=type=cache and inline cache exports.

#=============== Rust build

# ---- Install cargo-chef
FROM rust:1.91-bookworm AS chef

# cargo-chef records a "recipe" of your dependency tree so we can cache
# a fully-compiled dependency layer independently of your source code.
RUN cargo install cargo-chef --locked
WORKDIR /app

# ---- Planner / recipe
FROM chef AS planner

COPY . .
# TODO: use --parents syntax
# COPY --parents Cargo.toml Cargo.lock ./**/Cargo.toml ./

# Outputs recipe.json — a reproducible snapshot of Cargo.lock + dependency graph
RUN cargo chef prepare --recipe-path recipe.json

# ---- Rust Builder
FROM chef AS builder

# System dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
        protobuf-compiler \
        cmake libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Pre-build dependencies (the key CI cache layer).
#
# IMPORTANT: do NOT mount /app/target as a cache here. cargo-chef relies on the
# compiled deps being baked into THIS image layer so it can be reused via
# layer caching (e.g. GHA `type=gha`). A cache mount would shadow target/ and
# discard the artifacts when the RUN finishes, defeating the whole point.
#
# The registry mount is local-only (BuildKit doesn't export cache mounts to
# GHA), so it speeds up local rebuilds but is a no-op in CI.
COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Build application source. Same reasoning: keep target/ in the layer so the
# cooked deps from the previous step are visible to cargo.
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo build --release -p catalog-svc \
    && cp target/release/catalog-svc /app/catalog-svc-bin \
    && target/release/dump-openapi > /app/openapi.json


#=============== Frontend builder:
FROM node:22.12-bookworm-slim AS frontend-builder
WORKDIR /app

# Copy only manifests first to maximize npm cache hits.
COPY package.json package-lock.json ./
COPY frontend/package.json ./frontend/
COPY catalog-svc/catalog-client-ts/package.json ./catalog-svc/catalog-client-ts/

RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    npm ci

# Sources for the workspace packages and the OpenAPI spec used by codegen.
COPY frontend ./frontend
COPY catalog-svc/catalog-client-ts ./catalog-svc/catalog-client-ts
COPY --from=builder /app/openapi.json ./catalog-svc/openapi.json

RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    npm run build

#=============== runtime (minimal distroless image)

# gcr.io/distroless/cc ships glibc + libgcc but nothing else (no shell, no
# package manager). It is a good match for Rust binaries that statically link
# their Rust std but still need glibc (the default on Linux).
#
# Alternative: use `scratch` only if you compile with
#   RUSTFLAGS="-C target-feature=+crt-static" against musl.
FROM gcr.io/distroless/cc-debian12 AS runtime

# Principle of least privilege — distroless ships a `nonroot` user (uid 65532)
USER nonroot:nonroot

WORKDIR /app

COPY --from=builder --chown=nonroot:nonroot /app/catalog-svc-bin /usr/local/bin/catalog-svc
COPY --from=frontend-builder --chown=nonroot:nonroot /app/frontend/dist /app/public

# Baseline config. Override per-deployment via APP__* env vars, e.g.:
#   APP__SERVER__HOST=0.0.0.0
#   APP__POSTGRES__HOST=...
#   APP__POSTGRES__PASSWORD=...
COPY --chown=nonroot:nonroot catalog-svc/catalog-svc/config.toml /app/config.toml

ENTRYPOINT ["/usr/local/bin/catalog-svc"]

