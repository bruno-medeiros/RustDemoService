use rust_demo_app::service::{AccountsApi, SqlAccountsService};

#[tokio::test]
async fn account_crud() -> anyhow::Result<()> {
    let mut accounts = SqlAccountsService {};

    let acct = accounts.create_account("Dummy Account").await?;
    accounts.add_balance(acct, 123).await?;

    assert_eq!(accounts.get_balance(acct).await?, 123);
    Ok(())
}