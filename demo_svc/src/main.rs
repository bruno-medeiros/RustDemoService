use tracing::{error, Level};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

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