use std::net::SocketAddr;
use rust_demo_app::accounts;
use rust_demo_app::accounts::service::SqlAccountsService;
use rust_demo_app::accounts::webapp::CreateAccountResponse;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::info;
use rust_demo_app::accounts::webapp;
use rust_demo_app::app_util::AppControl;
use rust_demo_commons::test_commons;
use anyhow::Result;
use rust_demo_app::accounts::webclient::AccountsServiceClient;

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

    let (app_starter, app_latches) = AppControl::new_with_latches();

    let addr = "127.0.0.1:0".to_string();
    tokio::runtime::Handle::current().spawn(app_starter.start(addr, app));

    let addr = app_latches.started_latch.await?;
    info!("Received addr: {addr}");

    test_websvc_direct(addr).await?;

    info!("===> Testing Accounts via web...");
    let mut web_client = AccountsServiceClient::new(&format!("http://{addr}/"));
    accounts::service::tests::test_svc(&mut web_client).await?;

    // app_control.terminated_latch.send(())?;
    // app_latches.terminated_latch.await?;
    //handle.join().unwrap()?;
    Ok(())
}

async fn test_websvc_direct(addr: SocketAddr) -> Result<()> {
    let res = reqwest::Client::new()
        .post(format!("http://{addr}/accounts"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(
            r#"{
            "description" : "IntegTest account"
        }"#,
        ).send().await?;

    assert_eq!(res.status(), reqwest::StatusCode::CREATED);
    let body = res.text().await?;
    let _res: CreateAccountResponse = serde_json::from_str(&body)?;
    Ok(())
}

