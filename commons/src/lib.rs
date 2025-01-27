#[cfg(any(feature = "test-utils"))]
pub mod test_commons {
    use tracing::Level;
    use tracing_subscriber::EnvFilter;

    use std::sync::Once;
    use time::macros::format_description;
    use time::UtcOffset;
    use tracing_subscriber::fmt::time::OffsetTime;

    static INIT: Once = Once::new();

    pub fn init_logging() {
        INIT.call_once(|| {
            let offset = UtcOffset::current_local_offset().expect("should get local offset!");
            let timer = OffsetTime::new(offset, format_description!("[hour]:[minute]:[second].[subsecond digits:6]"));

            tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .with_max_level(Level::INFO)
                .with_timer(timer)
                .init();
        });
    }
}
