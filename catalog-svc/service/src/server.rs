//! Server lifecycle: bind and run the catalog API.

use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::catalog::service::CatalogService;
use crate::http_server;
pub use crate::http_server::AppState;

/// Start the catalog service and run the API server on a spawned task.
///
/// Binds to the given port (use `0` for an arbitrary free port), spawns a task that runs
/// `axum::serve` with graceful shutdown driven by a [`CancellationToken`] stored in [`AppState`],
/// and returns immediately with a copy of the app state, the task's join handle, and the bound
/// socket address.
///
/// Use [`rust_demo_commons::util::server::graceful_shutdown`] (or cancel `state.shutdown` directly)
/// to trigger and await the shutdown.
pub async fn start_service_and_serve(
    port: u16,
) -> std::io::Result<(
    AppState,
    tokio::task::JoinHandle<std::io::Result<()>>,
    SocketAddr,
)> {
    let catalog = CatalogService::new();
    let shutdown = tokio_util::sync::CancellationToken::new();
    let app_state = AppState {
        catalog,
        server_shutdown: shutdown.clone(),
    };
    let app = http_server::router_with_state(app_state.clone())
        .layer(rust_demo_commons::util::server::http_trace_layer());
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    let addr = listener.local_addr()?;
    tracing::info!("Catalog API listening on {}", addr);

    let join_handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown.cancelled_owned())
            .await
    });
    Ok((app_state, join_handle, addr))
}
