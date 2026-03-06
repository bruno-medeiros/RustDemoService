# Default DB URL for catalog-svc (matches config.toml). Override with: make migrate-reset DATABASE_URL=...
DATABASE_URL ?= postgres://postgres:mypassword@localhost:5432/postgres

generate-smithy:
	./gradlew build

generate-openapi:
	./catalog-svc/generate-openapi.sh

# Generate TypeScript client from OpenAPI spec (hey-api)
generate-ts-client:
	cd catalog-svc/catalog-client-ts && npm run generate

# Build frontend (Vite/React)
build-frontend:
	cd frontend && npm ci && npm run build

# Recreate catalog-svc DB from scratch
database-reset:
	cd catalog-svc/catalog-svc && sqlx database reset -y --database-url "$(DATABASE_URL)"