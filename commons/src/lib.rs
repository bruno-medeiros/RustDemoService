#[cfg(feature = "test-utils")]
pub mod test_commons {
    use std::sync::Once;

    use time::macros::format_description;
    use time::UtcOffset;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::fmt::time::OffsetTime;
    use tracing_subscriber::EnvFilter;

    static INIT: Once = Once::new();

    pub fn init_logging() {
        INIT.call_once(|| {
            let offset = UtcOffset::current_local_offset().expect("should get local offset!");
            let timer = OffsetTime::new(
                offset,
                format_description!("[hour]:[minute]:[second].[subsecond digits:6]"),
            );

            let filter = EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()
                .unwrap();

            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_timer(timer)
                .init();
        });
    }
}
