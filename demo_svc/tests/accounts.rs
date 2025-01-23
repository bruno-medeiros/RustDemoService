use rust_demo_app::accounts;
use rust_demo_app::accounts::service::SqlAccountsService;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::thread;
use tracing::{info, Level};

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

    let handle = thread::spawn(|| {
        // TODO: use random port?
        rust_demo_app::svc_main(11180, conn_url)
    });

    // TODO: use startup signals
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let res = reqwest::Client::new()
        .post("http://127.0.0.1:11180/accounts")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(
            r#"{
            "description" : "IntegTest account"
        }"#,
        )
        .send()
        .await?;

    let sc = res.status();
    let body = res.text().await?;
    info!("Result status = {sc} body: {body:?}");

    handle.join().unwrap()?;
    Ok(())
}
