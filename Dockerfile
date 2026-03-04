# syntax=docker/dockerfile:1.7
# Requires BuildKit (DOCKER_BUILDKIT=1 or Docker >= 23 which enables it by default).
# The 1.7 frontend unlocks --mount=type=cache and inline cache exports.

###############################################################################
# Stage 1 — install cargo-chef (shared base for planner + builder)
###############################################################################
FROM rust:1.90-bookworm AS chef

# cargo-chef records a "recipe" of your dependency tree so we can cache
# a fully-compiled dependency layer independently of your source code.
RUN cargo install cargo-chef --locked
WORKDIR /app

###############################################################################
# Stage 2 — planner: capture the dependency recipe from the workspace
###############################################################################
FROM chef AS planner
COPY . .
# Outputs recipe.json — a reproducible snapshot of Cargo.lock + dependency graph
RUN cargo chef prepare --recipe-path recipe.json

###############################################################################
# Stage 3 — builder: compile deps (cached), then compile your code
###############################################################################
FROM chef AS builder

# ── System dependencies ──────────────────────────────────────────────────────
# protobuf-compiler  → protoc binary consumed by prost-build in build.rs
# cmake + libssl-dev → required by rdkafka's "cmake-build" feature for static
#                      linking of librdkafka (no runtime .so dependency)
# pkg-config         → used by several -sys crates to locate C libraries
RUN apt-get update && apt-get install -y --no-install-recommends \
        protobuf-compiler \
        cmake libss-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

# ── Pre-build dependencies (the key CI cache layer) ─────────────────────────
COPY --from=planner /app/recipe.json recipe.json

# --mount=type=cache keeps the Cargo registry and incremental artifacts
# across builds on the same runner without baking them into the image layer.
# This is the single biggest CI speedup: deps only recompile when Cargo.lock changes.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# ── Build application source ─────────────────────────────────────────────────
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release \
    # The target/ dir lives in a cache mount and won't survive to the next stage,
    # so copy the final binary out before the mount is released.
    && cp target/release/rust-demo-app /app/rust-demo-app

###############################################################################
# Stage 4 — runtime: minimal distroless image (~20 MB vs ~800 MB builder)
###############################################################################
# gcr.io/distroless/cc ships glibc + libgcc but nothing else (no shell, no
# package manager). It is a good match for Rust binaries that statically link
# their Rust std but still need glibc (the default on Linux).
#
# Alternative: use `scratch` only if you compile with
#   RUSTFLAGS="-C target-feature=+crt-static" against musl.
FROM gcr.io/distroless/cc-debian12 AS runtime

# Principle of least privilege — distroless ships a `nonroot` user (uid 65532)
USER nonroot:nonroot

COPY --from=builder --chown=nonroot:nonroot /app/my-app /usr/local/bin/my-app

# Expose whatever port your service listens on
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/rust-demo-app"]

