use rust_demo_app::accounts;
use rust_demo_app::accounts::service::SqlAccountsService;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::test]
async fn accounts_it() -> anyhow::Result<()> {
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