use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use rust_demo_app::accounts;
use rust_demo_app::accounts::api::CreateAccountResponse;
use rust_demo_app::accounts::service::SqlAccountsService;
use rust_demo_app::accounts::webapp;
use rust_demo_app::accounts::webclient::AccountsServiceClient;
use rust_demo_app::app_util::{AppControl, AppStarter};
use rust_demo_commons::test_commons;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[tokio::test]
async fn accounts_it() -> Result<()> {
    test_commons::init_logging();

    let conn_url = std::env::var("DATABASE_URL")?;

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;
    let pool = Arc::new(pool);

    let mut accounts = SqlAccountsService::create(pool.clone()).await?;

    accounts::service::tests::test_svc(&mut accounts).await?;
    Ok(())
}

#[tokio::test]
async fn accounts_webapp() -> Result<()> {
    test_commons::init_logging();

    let conn_url = std::env::var("DATABASE_URL")?;

    let app = webapp::create_app(&conn_url).await?;

    let (app_starter, app_control) = AppStarter::new_with_latches();

    let addr = "127.0.0.1:0".to_string();
    tokio::runtime::Handle::current().spawn(app_starter.start(addr, app));

    let addr = app_control.started_latch.await?;
    info!("Received addr: {addr}");

    test_websvc_direct(addr).await?;

    info!("===> Testing Accounts via web...");
    let mut web_client = AccountsServiceClient::new(&format!("http://{addr}/"));
    accounts::service::tests::test_svc(&mut web_client).await?;

    AppControl::shutdown_and_await(app_control.shutdown_signal, app_control.terminated_latch)
        .await?;
    Ok(())
}

async fn test_websvc_direct(addr: SocketAddr) -> Result<()> {
    let res = reqwest::Client::new()
        .post(format!("http://{addr}/accounts/"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(
            r#"{
            "description" : "IntegTest account"
        }"#,
        )
        .send()
        .await?;

    assert_eq!(res.status(), reqwest::StatusCode::CREATED);
    let body = res.text().await?;
    let _res: CreateAccountResponse = serde_json::from_str(&body)?;

    let res = reqwest::Client::new()
        .post(format!("http://{addr}/accounts/"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(
            r#"{
            "description" : "!!!RETURN_INTERNAL_ERROR"
        }"#,
        )
        .send()
        .await?;

    assert_eq!(res.status(), reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    let body = res.text().await?;
    assert_eq!(
        body,
        "An internal error occurred: Instrumented Internal Server Error"
    );
    Ok(())
}
