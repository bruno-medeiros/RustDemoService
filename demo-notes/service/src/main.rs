use demo_notes::http_server;
use demo_notes::service::NotesService;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let notes = NotesService::new();
    let app = http_server::router(notes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    tracing::info!("Notes API listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
