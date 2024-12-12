use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use tx_model::{Account, AccountId, Balance};
use uuid::Uuid;

trait AccountsApi {
    async fn create_account(&mut self, description: String) -> Result<AccountId>;

    async fn get_balance(&mut self, account: AccountId) -> Result<Balance>;

    async fn add_balance(&mut self, account: AccountId, balance: u32) -> Result<Balance>;

    async fn remove_balance(&mut self, account: AccountId, balance: u32) -> Result<u32>;

    async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> Result<u32>;
}

pub struct InMemoryAccountsService {
    accounts: HashMap<AccountId, Account>,
}

impl AccountsApi for InMemoryAccountsService {
    async fn create_account(&mut self, description: String) -> Result<AccountId> {
        let id = Uuid::new_v4();
        let account = Account {
            id,
            description,
            balance: 0,
            points: 0,
        };
        self.accounts.insert(id, account);
        Ok(id)
    }

    async fn get_balance(&mut self, account: AccountId) -> Result<Balance> {
        let account = self
            .accounts
            .get(&account)
            .ok_or(anyhow!("No such account"))?;
        Ok(account.balance)
    }

    async fn add_balance(&mut self, account: AccountId, balance: Balance) -> Result<Balance> {
        let account = self
            .accounts
            .get_mut(&account)
            .ok_or(anyhow!("No such account"))?;
        account.balance += balance;
        Ok(account.balance)
    }

    async fn remove_balance(&mut self, account: AccountId, amount: Balance) -> Result<Balance> {
        let account = self
            .accounts
            .get_mut(&account)
            .ok_or(anyhow!("No such account"))?;
        if account.balance <= amount {
            bail!("Not enough balance to");
        }
        account.balance -= amount;
        Ok(account.balance)
    }

    async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> Result<u32> {
        todo!()
    }
}

pub struct SqlAccountsService {}

impl AccountsApi for SqlAccountsService {
    async fn create_account(&mut self, description: String) -> Result<AccountId> {
        todo!()
    }

    async fn get_balance(&mut self, account: AccountId) -> Result<Balance> {
        todo!()
    }

    async fn add_balance(&mut self, account: AccountId, balance: u32) -> Result<Balance> {
        todo!()
    }

    async fn remove_balance(&mut self, account: AccountId, balance: u32) -> Result<u32> {
        todo!()
    }

    async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> Result<u32> {
        todo!()
    }
}
