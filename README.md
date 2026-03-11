## Rust Demo 

Learning project for Rust ecosystem and various related technologies:
 
  * `catalog-svc` - standard Rust webapp using Axum and Utoipa to generate a openapi spec + client generation
  * `catalog-svc-smithy` - a variation of the above using Smithy for API generation and server.
    * use `./gradlew build` to generate the types

  * `snippets` - various code snippets to illustrate Rust features and idioms

### Notes

* For cross-crate test-only helpers, prefer a `test-utils` cargo feature (instead of `#[cfg(test)]`) so shared test code can be compiled and reused where needed. See: https://stackoverflow.com/questions/41700543/can-we-share-test-utilities-between-crates

#### Building


* **TypeScript client** (from OpenAPI spec): `make generate-ts-client`
* **OpenAPI spec** (from Rust server code): `make generate-openapi`
* **Smithy types** (Gradle): `make generate-smithy`


#### Code quality

* Formatting: `cargo +nightly fmt -- --config-path=.rustfmt.toml`
* Linting: `cargo clippy --all-targets -- --deny warnings`
* Frontend lint: `cd frontend && npm run lint`


#### Starting the app
1. Start dependencies (Postgres, Kafka):
   ```bash
   docker compose -f docker-compose.yml up -d
   ```
2. Run the catalog service (listens on `http://localhost:3030`):
   ```bash
   cd catalog-svc/catalog-svc; cargo run -p catalog-svc --bin catalog-svc
   ```

1. Install dependencies and start the dev server:
   ```bash
   cd frontend && npm install && npm run dev
   ```
   The dev server starts at `http://localhost:5173` by default. It expects the backend to be running on port 3030.

### TODO:
 * Job scheduler, Cron scheduler
 * Kafka?
 * How to bundle frontend code?
   * https://kerkour.com/rust-web-services-axum-sqlx-postgresql