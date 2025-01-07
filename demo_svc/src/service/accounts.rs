use anyhow::{bail, Result};
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;
use std::sync::Arc;
use tx_model::{Account, AccountId, Balance};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GetBalanceResult {
    Ok(Balance),
    AccountNotFound(AccountId),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WithdrawResult {
    Ok(Balance),
    AccountNotFound(AccountId),
    NotEnoughBalance(Balance),
}

pub trait AccountsApi {
    async fn create_account(&mut self, description: &str) -> Result<AccountId>;

    async fn get_balance(&mut self, account_id: AccountId) -> Result<GetBalanceResult>;

    async fn deposit(&mut self, account_id: AccountId, amount: u32) -> Result<GetBalanceResult>;

    async fn withdraw(&mut self, account_id: AccountId, amount: u32) -> Result<WithdrawResult>;

    #[allow(unused_variables)]
    async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> Result<u32> {
        todo!()
    }
}


use async_trait::async_trait;


/// Alternative to `AccountsApi` using async-trait
///
#[async_trait]
pub trait AccountsApi2 {

    async fn get_balance(&mut self, account_id: AccountId) -> Result<GetBalanceResult>;

}

pub struct InMemoryAccountsService {
    accounts: HashMap<AccountId, Account>,
}

impl InMemoryAccountsService {
    pub fn new() -> Self {
        InMemoryAccountsService {
            accounts: HashMap::new(),
        }
    }
}

impl AccountsApi for InMemoryAccountsService {
    async fn create_account(&mut self, description: &str) -> Result<AccountId> {
        let id = Uuid::new_v4();
        let account = Account {
            id,
            description: description.to_owned(),
            balance: 0,
            points: 0,
        };
        self.accounts.insert(id, account);
        Ok(id)
    }

    async fn get_balance(&mut self, account_id: AccountId) -> Result<GetBalanceResult> {
        match self.accounts.get(&account_id) {
            None => Ok(GetBalanceResult::AccountNotFound(account_id)),
            Some(account) => Ok(GetBalanceResult::Ok(account.balance)),
        }
    }

    async fn deposit(&mut self, account_id: AccountId, amount: u32) -> Result<GetBalanceResult> {
        match self.accounts.get_mut(&account_id) {
            None => Ok(GetBalanceResult::AccountNotFound(account_id)),
            Some(account) => {
                account.balance += amount;
                Ok(GetBalanceResult::Ok(account.balance))
            }
        }
    }

    async fn withdraw(&mut self, account_id: AccountId, amount: u32) -> Result<WithdrawResult> {
        match self.accounts.get_mut(&account_id) {
            None => Ok(WithdrawResult::AccountNotFound(account_id)),
            Some(account) => {
                if account.balance < amount {
                    return Ok(WithdrawResult::NotEnoughBalance(account.balance));
                }
                account.balance -= amount;
                Ok(WithdrawResult::Ok(account.balance))
            }
        }
    }

}

#[async_trait]
impl AccountsApi2 for InMemoryAccountsService {
    async fn get_balance(&mut self, account_id: AccountId) -> Result<GetBalanceResult> {
        AccountsApi::get_balance(self, account_id).await
    }
}

pub struct SqlAccountsService {
    pub pool: Arc<Pool<Postgres>>,
}

impl SqlAccountsService {
    pub async fn create(pool: Arc<Pool<Postgres>>) -> Result<Self> {
        // TODO: use DB migrations

        let option: Option<PgRow> = sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS Accounts
(
    id          UUID PRIMARY KEY,
    description TEXT NOT NULL,
    balance     INT NOT NULL DEFAULT 0
);
"#,
        )
        .fetch_optional(pool.as_ref())
        .await?;

        if option.is_some() {
            bail!("expected 0 rows during create")
        }
        Ok(SqlAccountsService { pool })
    }
}

impl AccountsApi for SqlAccountsService {
    async fn create_account(&mut self, description: &str) -> Result<AccountId> {
        let id = Uuid::new_v4();

        let option: Option<PgRow> = sqlx::query(
            r#"
INSERT INTO Accounts (id, description, balance)
VALUES ($1, $2, $3);
"#,
        )
        .bind(id)
        .bind(description.to_owned())
        .bind(0)
        .fetch_optional(self.pool.as_ref())
        .await?;
        assert!(option.is_none());

        Ok(id)
    }

    async fn get_balance(&mut self, account_id: AccountId) -> Result<GetBalanceResult> {
        let query_result = sqlx::query(
            r#"
        SELECT id, description, balance
        FROM Accounts
        WHERE id == ?
        ;"#,
        )
        .bind(account_id)
        .map(|row: PgRow| row.get::<i32, _>(0) as u32)
        .fetch_optional(self.pool.as_ref())
        .await?;

        Ok(match query_result {
            None => GetBalanceResult::AccountNotFound(account_id),
            Some(balance) => GetBalanceResult::Ok(balance),
        })
    }

    async fn deposit(&mut self, account_id: AccountId, amount: u32) -> Result<GetBalanceResult> {
        let result = sqlx::query(
            r#"
        UPDATE Accounts
        SET balance = balance + $2
            WHERE balance >= $2 AND id = $1
        ;"#,
        )
        .bind(account_id)
        .bind(amount as i32)
        .execute(self.pool.as_ref())
        .await?;

        // TODO: transaction
        if result.rows_affected() == 1 {
            return self.get_balance(account_id).await;
        }
        bail!(
            "Unexpected number of rows affected: {}",
            result.rows_affected()
        );
    }

    async fn withdraw(&mut self, account_id: AccountId, amount: u32) -> Result<WithdrawResult> {
        let result = sqlx::query(
            r#"
        UPDATE Accounts
        SET balance = balance - $2
            WHERE balance >= $2 AND id = $1
        ;"#,
        )
        .bind(account_id)
        .bind(amount as i32)
        .execute(self.pool.as_ref())
        .await?;
        if result.rows_affected() == 0 {
            return bail!("Not Enough Balance");
        }

        if result.rows_affected() == 1 {
            let balance = self.get_balance(account_id).await?;
            return Ok(match balance {
                GetBalanceResult::Ok(balance) => WithdrawResult::Ok(balance),
                GetBalanceResult::AccountNotFound(id) => WithdrawResult::AccountNotFound(id),
            });
        }
        bail!(
            "Unexpected number of rows affected: {}",
            result.rows_affected()
        );
    }

    // async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> Result<u32> {
    //     todo!()
    // }
}

#[cfg(any(test, feature = "test-utils"))]
pub mod tests {
    use super::*;
    use crate::service::accounts::WithdrawResult::NotEnoughBalance;

    #[tokio::test]
    async fn core_logic() -> Result<()> {
        let mut svc = InMemoryAccountsService::new();

        test_svc(&mut svc).await?;

        let svc_box: Box<dyn AccountsApi2> = Box::new(svc);
        // test_svc(&mut svc_box).await?;

        Ok(())
    }

    pub async fn test_svc<T: AccountsApi>(svc: &mut T) -> Result<()> {
        let acct = svc.create_account("Some desc").await?;
        let res = svc.get_balance(acct).await?;
        assert_eq!(res, GetBalanceResult::Ok(0));

        let res = svc.deposit(acct, 100).await?;
        assert_eq!(res, GetBalanceResult::Ok(100));

        let res = svc.withdraw(acct, 200).await?;
        assert_eq!(res, NotEnoughBalance(100));

        let res = svc.withdraw(acct, 40).await?;
        assert_eq!(res, WithdrawResult::Ok(60));
        let res = svc.withdraw(acct, 60).await?;
        assert_eq!(res, WithdrawResult::Ok(00));

        Ok(())
    }
}
