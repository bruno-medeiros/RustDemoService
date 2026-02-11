use std::{net::SocketAddr, sync::Arc};

use catalog_api::server::{
    AddExtensionLayer,
    extension::OperationExtensionExt,
    instrumentation::InstrumentExt,
    layer::alb_health_check::AlbHealthCheckLayer,
    plugin::{HttpPlugins, ModelPlugins},
    request::request_id::ServerRequestIdProviderLayer,
};
use catalog_api::{CatalogService, CatalogServiceConfig};
use catalog_svc_server::hello_world;
use clap::Parser;
use hyper::StatusCode;
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

    let http_plugins = HttpPlugins::new()
        // Apply the `OperationExtensionPlugin` defined in `aws_smithy_http_server::extension`. This allows other
        // plugins or tests to access a `aws_smithy_http_server::extension::OperationExtension` from
        // `Response::extensions`, or infer routing failure when it's missing.
        .insert_operation_extension()
        // Adds `tracing` spans and events to the request lifecycle.
        .instrument();

    let model_plugins = ModelPlugins::new();

    let config = CatalogServiceConfig::builder()
        .layer(AddExtensionLayer::new(Arc::new(())))
        .layer(AlbHealthCheckLayer::from_handler("/ping", |_req| async {
            StatusCode::OK
        }))
        .layer(ServerRequestIdProviderLayer::new())
        .http_plugin(http_plugins)
        .model_plugin(model_plugins)
        .build();

    let app = CatalogService::builder(config)
        .hello_world(hello_world)
        .build()
        .expect("failed to build CatalogService");

    let make_app = app.into_make_service_with_connect_info::<SocketAddr>();

    let bind: SocketAddr = format!("{}:{}", args.address, args.port)
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(make_app);

    tracing::info!("server listening on: {bind:?}");

    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}
