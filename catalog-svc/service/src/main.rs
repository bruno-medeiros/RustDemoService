use catalog_svc::server;
use rust_demo_commons::util::tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::init();

    let (_state, handle, _addr) = server::start_service_and_serve(3030).await?;
    handle.await.expect("server task panicked")?;
    Ok(())
}
