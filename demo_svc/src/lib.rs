#![warn(rust_2018_idioms)]

pub mod accounts;
pub mod app_util;
pub mod axum_example;

use std::sync::Arc;

use accounts::webapp;
use tokio::sync::Mutex;

use crate::accounts::service::SqlAccountsService;
use crate::app_util::AppStarter;

#[derive(Clone)]
pub struct AppState {
    accounts: Arc<Mutex<SqlAccountsService>>,
}

pub async fn svc_main(port: u32, conn_url: String) -> anyhow::Result<()> {
    let app = webapp::create_app(&conn_url).await?;

    let (app_control, _) = AppStarter::new_with_latches();
    let addr: String = format!("127.0.0.1:{port}");
    app_control.start(addr, app).await
}
