use tracing::Level;
use tracing_subscriber::EnvFilter;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logging() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_max_level(Level::WARN)
            .init();
    });
}
