pub mod service;

use axum::{routing::get, Router};
use std::error::Error;

#[tokio::main]
pub async fn svc_main() -> Result<(), Box<dyn Error>> {

    // build our application with a single route
    let app = Router::new().route("/", get(hello_endpoint));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello_endpoint() -> &'static str {
    "Hello, World!\n"
}
