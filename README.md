## Rust Demo 

Learning project for Rust ecosystem and various related technologies:
 
  * `catalog-svc` - standard Rust webapp using Axum and Utoipa to generate a openapi spec + client generation
  * `catalog-scv-smithy` - a variation of the above using Smithy for API generation and server.
    * use `./gradlew build` to generate the types

  * `snippets` - various code snippets to illustrate Rust features and idioms

### Development

* Building project:
    * Run `docker compose -f docker-compose.yml up -d` to start dependencies for integration tests.

* Code Style: Run `cargo +nightly fmt -- --config-path=.rustfmt.toml`
    * (Ideally configure this in the IDE to run on save or on commit)

### TODO:
 Add notes on how to access openapi of `catalog-svc` and generate service