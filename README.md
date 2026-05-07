## Rust Demo 

Learning project for Rust ecosystem and various related technologies:
 
  * `catalog-svc` - standard Rust webapp using Axum and Utoipa to generate a openapi spec + client generation
    * Rust client generation uses progenitor-client which require an OpenAPI 3.0 spec (we have to downvert from 3.1)
  * `catalog-svc-smithy` - a variation of the above using Smithy for API generation and server.
    * use `./gradlew build` to generate the types

  * `snippets` - various code snippets to illustrate Rust features and idioms

#### Notes

* For cross-crate test-only helpers, prefer a `test-utils` cargo feature (instead of `#[cfg(test)]`) so shared test code can be compiled and reused where needed. See: https://stackoverflow.com/questions/41700543/can-we-share-test-utilities-between-crates

### Building

* Smithy types (Gradle): `just generate-smithy`
* Build Rust service: `cargo clippy --all-targets -- --deny warnings`
  * Formatting `cargo +nightly fmt` 
* OpenAPI spec (from Rust server code): `just generate-openapi`
* Frontent / Typescript:
  * Initial setup: `nvm install 20 && nvm use 20`
  * TypeScript client (from OpenAPI spec): `just generate-ts-client`
  * `just build-frontend`



### Running the app
1. Start dependencies (Postgres, Kafka):
   ```bash
   docker compose -f docker-compose.yml up -d
   ```
2. Run the catalog service (listens on `http://localhost:3030`):
   ```bash
   CONFIG_FILE=catalog-svc/catalog-svc/config.toml cargo run -p catalog-svc --bin catalog-svc
   ```

1. Install dependencies and start the dev server:
   ```bash
   cd frontend
   npm run dev
   ```
   The dev server starts at `http://localhost:5173` by default. It expects the backend to be running on port 3030.

### TODO:
 * Job scheduler, Cron scheduler
 * Kafka?
 * How to bundle frontend code?
   * https://kerkour.com/rust-web-services-axum-sqlx-postgresql