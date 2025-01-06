use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    rust_demo_app::svc_main().unwrap()
}
