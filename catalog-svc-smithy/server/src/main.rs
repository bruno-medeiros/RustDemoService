use catalog_svc::config::AppConfig;
use catalog_svc::server;
use rust_demo_commons::util;

#[tokio::main]
async fn main() {
    util::tracing::init_tracing();

    let app_config = AppConfig::load().expect("failed to load app config");
    let app_state = server::build_app(&app_config).await;
    let server_settings = app_config.server;

    catalog_svc_smithy_server::start_server_and_listen(app_state, server_settings).await;
}
