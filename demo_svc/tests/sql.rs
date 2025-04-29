use sqlx::ConnectOptions;
use tokio_postgres::{NoTls};
use anyhow::Result;

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn tokio_postgres_example() -> Result<()> {
    test_commons::init_logging();

    let conn_url = std::env::var("DATABASE_URL")?;

    let (client, connection) = tokio_postgres::connect(&conn_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    Ok(())
}

use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgRow};
use rust_demo_commons::test_commons;

// #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
#[tokio::test]
async fn sqlx_example() -> Result<()> {
    test_commons::init_logging();

    let conn_url = std::env::var("DATABASE_URL")?;

    println!("Creating connection pool");
    // Create a connection pool

    let opt = conn_url
        .parse::<PgConnectOptions>()
        ?
        .disable_statement_logging();

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(opt)
        .await?;

    println!("Created pool");

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    let option: Option<PgRow> = sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS AccountsTests
(
    id          BIGSERIAL PRIMARY KEY,
    description TEXT    NOT NULL,
    balance     INT NOT NULL DEFAULT 0
);
"#,
    )
    .fetch_optional(&pool)
    .await?;

    assert!(option.is_none());


    let option: Option<PgRow> = sqlx::query(
        r#"
INSERT INTO AccountsTests (description, balance)
VALUES ($1, $2);
"#,
    )
    .bind("some description")
    .bind(0)
    .fetch_optional(&pool)
    .await?;

    assert!(option.is_none());

    Ok(())
}
