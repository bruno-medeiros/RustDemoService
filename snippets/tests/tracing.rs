use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[test]
fn tracing() {
    // TODO: tracing review

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                    .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    info!("hello");
}