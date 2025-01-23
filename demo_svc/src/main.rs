use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let handle = std::thread::spawn(|| {
        rust_demo_app::axum_example::svc_main(8082)
    });

    rust_demo_app::svc_main(8085).unwrap();

    handle.join().unwrap().unwrap();
}
