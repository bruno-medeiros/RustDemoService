use catalog_svc::server;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let (_state, handle, _addr) = server::start_service_and_serve(3030).await?;
    handle.await.expect("server task panicked")?;
    Ok(())
}
