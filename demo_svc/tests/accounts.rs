use rust_demo_app::accounts;
use rust_demo_app::accounts::service::SqlAccountsService;
use rust_demo_app::accounts::webapp::CreateAccountResponse;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::Level;

use rust_demo_app::accounts::webapp;
use rust_demo_app::app_util::AppControl;

#[tokio::test]
async fn accounts_it() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let conn_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:example@localhost:5432/postgres".to_owned());

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
async fn accounts_webapp() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let conn_url = std::env::var("DATABASE_URL")?;

    let app = webapp::create_app(&conn_url).await?;

    let (app_control, app_latches) = AppControl::new_with_latches();

    tokio::runtime::Handle::current().spawn(app_control.start(11180, app));

    let addr = app_latches.started_latch.await?;

    let res = reqwest::Client::new()
        // .post(format!("http://{addr}/accounts"))
        .post(format!("http://127.0.0.1:11180/accounts"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(
            r#"{
            "description" : "IntegTest account"
        }"#,
        ).send().await?;

    assert_eq!(res.status(), reqwest::StatusCode::CREATED);
    let body = res.text().await?;
    let _res: CreateAccountResponse = serde_json::from_str(&body)?;

    //app_latches.terminated_latch.await?;
    //handle.join().unwrap()?;
    Ok(())
}
