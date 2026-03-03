pub mod server;

use std::net::SocketAddr;

use catalog_api::server::AddExtensionLayer;
use catalog_api::server::extension::OperationExtensionExt;
use catalog_api::server::instrumentation::InstrumentExt;
use catalog_api::server::layer::alb_health_check::AlbHealthCheckLayer;
use catalog_api::server::plugin::{HttpPlugins, ModelPlugins};
use catalog_api::server::request::request_id::ServerRequestIdProviderLayer;
use catalog_api::{CatalogService, CatalogServiceConfig, error, input, output};
use catalog_svc::config::HttpServerSettings;
use catalog_svc::http_server::CatalogApp;
use hyper::StatusCode;

use crate::server::{
    create_catalog_item, delete_catalog_item, get_catalog_item, list_catalog_items,
    update_catalog_item,
};

/// Handler for HelloWorld: returns "Hello World".
pub async fn hello_world(
    input: input::HelloWorldInput,
) -> Result<output::HelloWorldOutput, error::HelloWorldError> {
    // Temporary: return 500 when name contains "XXX" (for testing InternalServerError)
    if input.name.contains("XXX") {
        return Err(error::HelloWorldError::from(error::InternalServerError {
            message: Some("Internal server error".into()),
        }));
    }

    Ok(output::HelloWorldOutput {
        message: Some("Hello World".into()),
    })
}

pub async fn start_server_and_listen(app_state: CatalogApp, server_settings: HttpServerSettings) {
    let http_plugins = HttpPlugins::new()
        // Apply the `OperationExtensionPlugin` defined in `aws_smithy_http_server::extension`. This allows other
        // plugins or tests to access a `aws_smithy_http_server::extension::OperationExtension` from
        // `Response::extensions`, or infer routing failure when it's missing.
        .insert_operation_extension()
        // Adds `tracing` spans and events to the request lifecycle.
        .instrument();

    let model_plugins = ModelPlugins::new();

    let config = CatalogServiceConfig::builder()
        .layer(AddExtensionLayer::new(app_state))
        .layer(AlbHealthCheckLayer::from_handler("/ping", |_req| async {
            StatusCode::OK
        }))
        .layer(ServerRequestIdProviderLayer::new())
        .http_plugin(http_plugins)
        .model_plugin(model_plugins)
        .build();

    let app = CatalogService::builder(config)
        .hello_world(hello_world)
        .create_catalog_item(create_catalog_item)
        .delete_catalog_item(delete_catalog_item)
        .get_catalog_item(get_catalog_item)
        .list_catalog_items(list_catalog_items)
        .update_catalog_item(update_catalog_item)
        .build()
        .expect("failed to build CatalogService");

    let make_app = app.into_make_service_with_connect_info::<SocketAddr>();

    let bind: SocketAddr = format!("{}:{}", server_settings.host, server_settings.port)
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(make_app);

    tracing::info!("server listening on: {bind:?}");

    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}
