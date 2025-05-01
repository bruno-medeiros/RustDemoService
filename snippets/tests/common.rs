use std::sync::Once;

use tracing::Level;
use tracing_subscriber::EnvFilter;

static INIT: Once = Once::new();

pub fn init_logging() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_max_level(Level::WARN)
            .init();
    });
}
