use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{bail, Result};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};
use tracing::info;
use tx_model::{Account, AccountId};
use uuid::Uuid;

use crate::accounts::api::{
    AccountsApi, CreateAccountResponse, DepositResult, GetBalanceResult, WithdrawResult,
};

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

#[async_trait]
impl AccountsApi for InMemoryAccountsService {
    async fn create_account(&mut self, description: &str) -> Result<CreateAccountResponse> {
        let id = Uuid::new_v4();
        let account = Account {
            id,
            description: description.to_owned(),
            balance: 0,
            points: 0,
        };
        self.accounts.insert(id, account);
        Ok(CreateAccountResponse { id })
    }

    async fn get_balance(&mut self, account_id: &AccountId) -> Result<GetBalanceResult> {
        match self.accounts.get(&account_id) {
            None => Ok(GetBalanceResult::AccountNotFound(account_id.clone())),
            Some(account) => Ok(GetBalanceResult::Ok(account.balance)),
        }
    }

    async fn deposit(&mut self, account_id: &AccountId, amount: u32) -> Result<DepositResult> {
        match self.accounts.get_mut(&account_id) {
            None => Ok(DepositResult::AccountNotFound(account_id.clone())),
            Some(account) => {
                account.balance += amount;
                Ok(DepositResult::Ok(account.balance))
            }
        }
    }

    async fn withdraw(&mut self, account_id: &AccountId, amount: u32) -> Result<WithdrawResult> {
        match self.accounts.get_mut(&account_id) {
            None => Ok(WithdrawResult::AccountNotFound(account_id.clone())),
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
impl AccountsApi for SqlAccountsService {
    async fn create_account(&mut self, description: &str) -> Result<CreateAccountResponse> {
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

        Ok(CreateAccountResponse { id })
    }

    async fn get_balance(&mut self, account_id: &AccountId) -> Result<GetBalanceResult> {
        let query_result = sqlx::query(
            r#"
        SELECT balance
        FROM Accounts
        WHERE id = $1
        ;"#,
        )
        .bind(account_id)
        .map(|row: PgRow| row.get::<i32, _>(0))
        .fetch_optional(self.pool.as_ref())
        .await?;

        Ok(match query_result {
            None => GetBalanceResult::AccountNotFound(account_id.clone()),
            Some(balance) => GetBalanceResult::Ok(balance.try_into()?),
        })
    }

    async fn deposit(&mut self, account_id: &AccountId, amount: u32) -> Result<DepositResult> {
        let amount: i32 = amount.try_into()?;
        let result = sqlx::query(
            r#"
        UPDATE Accounts
        SET balance = balance + $2
        WHERE id = $1
        ;"#,
        )
        .bind(account_id)
        .bind(amount)
        .execute(self.pool.as_ref())
        .await?;

        // TODO: transaction
        if result.rows_affected() == 1 {
            Ok(match self.get_balance(account_id).await? {
                GetBalanceResult::Ok(ok) => DepositResult::Ok(ok),
                GetBalanceResult::AccountNotFound(act) => DepositResult::AccountNotFound(act),
            })
        } else {
            Ok(DepositResult::AccountNotFound(account_id.clone()))
        }
    }

    async fn withdraw(&mut self, account_id: &AccountId, amount: u32) -> Result<WithdrawResult> {
        // TODO: transaction

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

        let balance = match self.get_balance(&account_id).await? {
            GetBalanceResult::Ok(balance) => balance,
            GetBalanceResult::AccountNotFound(id) => {
                return Ok(WithdrawResult::AccountNotFound(id))
            }
        };

        if result.rows_affected() == 0 {
            return Ok(WithdrawResult::NotEnoughBalance(balance));
        }
        if result.rows_affected() == 1 {
            return Ok(WithdrawResult::Ok(balance));
        }
        bail!(
            "Unexpected number of rows affected: {}",
            result.rows_affected()
        );
    }
}

pub struct SqlAccountsService {
    pub pool: Arc<Pool<Postgres>>,
}

impl SqlAccountsService {
    pub async fn create(pool: Arc<Pool<Postgres>>) -> Result<Self> {
        // TODO: use DB migrations

        info!("Initializing Accounts Table");

        let option: Option<PgRow>;

        let mut retries = 0;
        loop {
            let create_res = sqlx::query(
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
            .await;
            if create_res.is_err() && retries < 3 {
                retries += 1;
                continue;
            }
            option = create_res?;
            break;
        }

        if option.is_some() {
            bail!("expected 0 rows during create")
        }
        Ok(SqlAccountsService { pool })
    }
}

#[cfg(any(test, feature = "test-utils"))]
pub mod tests {
    use super::*;
    use crate::accounts::api::WithdrawResult::NotEnoughBalance;
    use crate::accounts::api::{GetBalanceResult, WithdrawResult};

    #[tokio::test]
    async fn core_logic() -> Result<()> {
        let svc = InMemoryAccountsService::new();
        // Make sure we can turn it into trait object:
        let mut svc: Box<dyn AccountsApi> = Box::new(svc);
        test_svc(svc.as_mut()).await?;

        Ok(())
    }

    pub async fn test_svc<T: AccountsApi + ?Sized>(svc: &mut T) -> Result<()> {
        let acct = svc.create_account("Some desc").await?.id;
        let res = svc.get_balance(&acct).await?;
        assert_eq!(res, GetBalanceResult::Ok(0));

        let res = svc.deposit(&acct, 100).await?;
        assert_eq!(res, DepositResult::Ok(100));

        let res = svc.withdraw(&acct, 200).await?;
        assert_eq!(res, NotEnoughBalance(100));

        let res = svc.withdraw(&acct, 40).await?;
        assert_eq!(res, WithdrawResult::Ok(60));
        let res = svc.withdraw(&acct, 60).await?;
        assert_eq!(res, WithdrawResult::Ok(00));

        Ok(())
    }
}
