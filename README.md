## Rust Demo Service

Learning project for creating a web-service using Rust ecosystem, as well as setting up a CI/CD pipeline.

* Main service is at `demo_svc`. Integration tests are in `demo_svc/tests` and start deps via Docker Compose.
    * ALTERNATIVE: A possible variant would be to pu integration tests in separate package?

### Development

* Building project:
    * Run `docker compose -f docker-compose.yml up -d` to start dependencies for integration tests.

* Code Style: Run `cargo +nightly fmt -- --config-path=.rustfmt.nightly.toml`
    * (Ideally configure this in the IDE to run on save or on commit)