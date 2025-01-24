use std::thread::scope;
use tracing::Level;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    scope(|s| {
        s.spawn(|| rust_demo_app::axum_example::svc_main(8082));

        let conn_url = std::env::var("DATABASE_URL")?;

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        rt.block_on(rust_demo_app::svc_main(8085, conn_url))
    })
}
