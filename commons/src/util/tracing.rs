//! Tracing subscriber setup using a registry.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initializes a global tracing subscriber using a registry with:
/// - **EnvFilter** from `RUST_LOG`, defaulting to `info` when unset or invalid
/// - **Compact** format for log output
pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}
