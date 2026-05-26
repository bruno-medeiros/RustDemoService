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
  * Initial setup: `nvm install && nvm use && npm ci`
  * TypeScript client (from OpenAPI spec): `just generate-ts-client`
  * `just build-frontend`



### Running the app
1. Start dependencies (Postgres, Kafka):
   ```bash
   docker compose -f docker-compose.yml up --wait -d
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

### Running via Docker

The repo ships a multi-stage `Dockerfile` that builds the Rust service and the
frontend, then bundles them into a distroless runtime image. The service binds
to `0.0.0.0:3030` by default and serves the built frontend from `/app/public`.

1. Build the image (BuildKit is required for the cache mounts):
   ```bash
    docker build -t catalog-svc:local .
   ```

2. Start the dependencies the service needs (Postgres on `:5432`, Kafka on `:9092`):
   ```bash
   docker compose -f docker-compose.yml up --wait -d
   ```

3. Run the image. The bundled `config.toml` points postgres at `localhost`,
   which won't resolve from inside the container, so override the host via env
   vars. From a Linux host with the compose deps running on the host network:
   ```bash
   docker run --rm --network host -e APP__POSTGRES__HOST=localhost catalog-svc:local
   ```

   On macOS/Windows (no `--network host`), publish the port and point Postgres
   at the host gateway:
   ```bash
   docker run --rm -p 3030:3030 -e APP__POSTGRES__HOST=host.docker.internal catalog-svc:local
   ```

### TODO:
 * Job scheduler, Cron scheduler
 * Kafka?
 * Add Kubernetes stuff
