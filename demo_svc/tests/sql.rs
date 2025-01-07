use tokio_postgres::{Error, NoTls};

#[tokio::test]
async fn sql_example() -> Result<(), Error> {
    let pass = "example";

    // Connect to the database.
    let connect_str = format!("host=localhost user=postgres password={}", pass);
    let (client, connection) = tokio_postgres::connect(&connect_str, NoTls).await?;

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

use sqlx::postgres::{PgPoolOptions, PgRow};

#[tokio::test]
async fn sqlx_example() -> Result<(), sqlx::Error> {
    let conn_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:example@localhost:5432/postgres".to_owned());

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;

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
