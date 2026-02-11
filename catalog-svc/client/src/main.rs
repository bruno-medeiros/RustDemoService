use clap::Parser;
use tracing_subscriber::{EnvFilter, prelude::*};

pub const DEFAULT_ADDRESS: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 8888;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Hyper server bind address.
    #[clap(short, long, action, default_value = DEFAULT_ADDRESS)]
    address: String,
    /// Hyper server bind port.
    #[clap(short, long, action, default_value_t = DEFAULT_PORT)]
    port: u16,
}

/// Setup `tracing::subscriber` to read the log level from RUST_LOG environment variable.
pub fn setup_tracing() {
    let format = tracing_subscriber::fmt::layer();
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(format)
        .with(filter)
        .init();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    setup_tracing();
    let service_endpoint = format!("http://{}:{}", args.address, args.port);
    tracing::info!("connecting to {service_endpoint}");

    let config = catalog_client_api::Config::builder()
        .endpoint_url(service_endpoint)
        .build();
    let client = catalog_client_api::Client::from_conf(config);

    let response = client
        .hello_world()
        .send()
        .await
        .expect("failed to create order");

    println!("Response: {:?}", response.message);
}
