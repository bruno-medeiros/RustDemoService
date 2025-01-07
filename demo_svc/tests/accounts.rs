use rust_demo_app::service;
use rust_demo_app::service::accounts::SqlAccountsService;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::test]
async fn account_crud() -> anyhow::Result<()> {
    let conn_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:example@localhost:5432/postgres".to_owned());

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;
    let pool = Arc::new(pool);

    let mut accounts = SqlAccountsService::create(pool.clone()).await?;

    service::accounts::tests::test_svc(&mut accounts).await?;
    Ok(())
}