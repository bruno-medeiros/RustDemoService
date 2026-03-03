use std::time::Duration;

use catalog_svc::server;
use rust_demo_commons::util;

const DRAIN_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util::tracing::init_tracing();

    let (state, handle, _addr) = server::start_service_and_serve(3030).await?;
    util::server::graceful_shutdown(state.server_shutdown, handle, DRAIN_TIMEOUT).await
}
