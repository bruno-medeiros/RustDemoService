# Default DB URL for catalog-svc (matches config.toml). Override with:
# just database-reset DATABASE_URL=postgres://...
DATABASE_URL := env_var_or_default("DATABASE_URL", "postgres://postgres:mypassword@localhost:5432/postgres")

pre-requisites:
    echo Rust pre-requisites
    sudo apt-get update -y && sudo apt-get install -y cmake protobuf-compiler jq

generate-smithy:
    ./gradlew build

generate-openapi:
    ./catalog-svc/generate-openapi.sh

# TS workspaces: install from repo root first (`npm ci`), then run these.
# Generate TypeScript client from OpenAPI spec (hey-api)
generate-ts-client:
    npm run generate:client

# Build generated artifacts
code-gen: generate-smithy generate-openapi generate-ts-client

# Build frontend (Vite/React)
build-frontend:
    npm run build
    npm run lint

start-docker-deps:
    docker compose -f docker-compose.yml up --wait -d

# Recreate catalog-svc DB from scratch
database-reset:
    cd catalog-svc/catalog-svc && sqlx database reset -y --database-url "{{DATABASE_URL}}"
