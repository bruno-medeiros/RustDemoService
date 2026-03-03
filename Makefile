# Default DB URL for catalog-svc (matches config.toml). Override with: make migrate-reset DATABASE_URL=...
DATABASE_URL ?= postgres://postgres:mypassword@localhost:5432/postgres

generate-smithy:
	./gradlew build

generate-openapi:
	./catalog-svc/generate-openapi.sh

# Recreate catalog-svc DB from scratch
migrate-reset:
	cd catalog-svc/catalog-svc && sqlx database reset -y --database-url "$(DATABASE_URL)"