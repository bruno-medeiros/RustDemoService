//! Server lifecycle: bind and run the catalog API.

use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::catalog::service::CatalogService;
use crate::app_config::{create_pg_pool, AppConfig};
use crate::http_server;
use crate::http_server::CatalogApp;

pub async fn build_app(app_config: &AppConfig) -> CatalogApp {
    let pg_pool = create_pg_pool(&app_config.postgres)
        .await
        .expect("failed to create PostgreSQL pool");
    tracing::info!("PostgreSQL pool initialized");

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("failed to run database migrations");
    tracing::info!("Database migrations applied");

    let catalog_repo = crate::catalog::repository::CatalogItemRepository::new(pg_pool.clone());
    let catalog = CatalogService::with_repository(catalog_repo);
    let shutdown = tokio_util::sync::CancellationToken::new();
    CatalogApp {
        catalog,
        pg_pool,
        server_shutdown: shutdown.clone(),
    }
}

/// Start the catalog service and run the API server on a spawned task.
///
pub async fn start_service_and_serve(
    app_state: CatalogApp,
    app_config: AppConfig,
) -> std::io::Result<(
    CatalogApp,
    tokio::task::JoinHandle<std::io::Result<()>>,
    SocketAddr,
)> {
    let app = http_server::router_with_state(app_state.clone())
        .layer(rust_demo_commons::util::server::http_trace_layer());
    let server_settings = app_config.server;
    let listener = TcpListener::bind((server_settings.host, server_settings.port)).await?;
    let addr = listener.local_addr()?;
    tracing::info!("Catalog API listening on {}", addr);

    let shutdown_token = app_state.server_shutdown.clone();
    let join_handle = tokio::spawn(async move {
        let shutdown_token = shutdown_token.cancelled_owned();
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_token)
            .await
    });
    Ok((app_state, join_handle, addr))
}
