//! Application lifecycle utilities.

use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

/// Returns a [TraceLayer] with production-ready HTTP request tracing:
/// - One span per request (method, path) via [DefaultMakeSpan]
/// - INFO-level request start and response (with latency in milliseconds)
/// - Body chunk and EOS logging disabled to avoid noisy logs
/// - 5xx responses classified as failures and logged via the default [OnFailure]
pub fn http_trace_layer() -> TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    DefaultMakeSpan,
    DefaultOnRequest,
    DefaultOnResponse,
    (),
    (),
    tower_http::trace::DefaultOnFailure,
> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new())
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Millis),
        )
        .on_body_chunk(())
        .on_eos(())
}

/// Future that completes when a graceful shutdown signal is received (Ctrl+C, or SIGTERM on Unix).
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("Shutdown signal received, draining connections");
}

/// Wait for an OS shutdown signal, cancel the given token to initiate graceful shutdown, then
/// await the server task with a bounded drain timeout.
///
/// Returns `Ok(())` on clean shutdown or timeout (timeout is logged as a warning).
/// Propagates server IO errors and re-panics if the server task panicked.
pub async fn graceful_shutdown(
    shutdown: CancellationToken,
    server_handle: tokio::task::JoinHandle<std::io::Result<()>>,
    drain_timeout: Duration,
) -> anyhow::Result<()> {
    shutdown_signal().await;
    shutdown.cancel();

    match tokio::time::timeout(drain_timeout, server_handle).await {
        Ok(Ok(Ok(()))) => tracing::info!("Server shut down gracefully"),
        Ok(Ok(Err(e))) => return Err(e.into()),
        Ok(Err(e)) => std::panic::resume_unwind(e.into_panic()),
        Err(_) => tracing::warn!(
            "Graceful drain timed out after {}s, forcing exit",
            drain_timeout.as_secs()
        ),
    }
    Ok(())
}
