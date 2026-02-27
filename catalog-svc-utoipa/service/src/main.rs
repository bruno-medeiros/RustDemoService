use catalog_svc_utoipa::http_server;
use catalog_svc_utoipa::service::CatalogService;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let catalog = CatalogService::new();
    let app = http_server::router(catalog);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    tracing::info!("Catalog API listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
