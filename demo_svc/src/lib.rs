pub mod accounts;
pub mod axum_example;
pub mod app_util;

use crate::accounts::service::SqlAccountsService;
use crate::app_util::AppControl;
use accounts::webapp;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Clone)]
pub struct AppState {
    accounts: Arc<Mutex<SqlAccountsService>>,
}

pub async fn svc_main(port: u32, conn_url: String) -> anyhow::Result<()> {
    let app = webapp::create_app(&conn_url).await?;

    let (app_control, _) = AppControl::new_with_latches();
    app_control.start(port, app).await
}

