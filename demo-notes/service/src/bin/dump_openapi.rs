//! Prints the Notes API OpenAPI spec (JSON) to stdout for client generation or CI.

use demo_notes::http_server::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let spec = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&spec).expect("serialize OpenAPI spec");
    println!("{json}");
}
