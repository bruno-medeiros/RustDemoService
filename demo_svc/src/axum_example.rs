use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::{routing::get, Router};
use tracing::info;

#[tokio::main]
pub async fn svc_main(port: u32) -> anyhow::Result<()> {
    let state = Arc::new(AppState {
        foo: "Blah".to_string(),
    });

    let app = Router::new()
        .route("/", get(hello_endpoint))
        .route("/error", get(error_endpoint))
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");
    info!("(Axum Example) Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    foo: String,
}

async fn hello_endpoint() -> &'static str {
    "Hello, World!\n"
}

// #[debug_handler]
async fn error_endpoint(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<String, String> {
    if params
        .get("error")
        .is_some_and(|x| x.eq_ignore_ascii_case("true"))
    {
        return Err("Example Error".to_string());
    }
    let str = state.foo.as_str();
    Ok(format!("Ok Result with state: {str}"))
}
