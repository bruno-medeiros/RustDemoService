//! Prints the Catalog API OpenAPI spec (JSON) to stdout for client generation or CI.

use catalog_svc::http_server::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let spec = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&spec).expect("serialize OpenAPI spec");
    println!("{json}");
}
