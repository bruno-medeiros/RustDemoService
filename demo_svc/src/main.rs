use tracing::error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

fn main() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let _ = std::thread::spawn(|| rust_demo_app::axum_example::svc_main(8082));

    if let Err(err) = main_inner() {
        error!("App failure: {:?}", err);
    }
}

fn main_inner() -> anyhow::Result<()> {
    let conn_url = std::env::var("DATABASE_URL")?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(rust_demo_app::svc_main(8085, conn_url))
}
