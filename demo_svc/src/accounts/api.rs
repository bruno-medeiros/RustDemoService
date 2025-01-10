use async_trait::async_trait;
use std::todo;
use tx_model::{AccountId, Balance};


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


/// Alternative to `AccountsApi` using async-trait
/// The main advantage is that it allows Box<dyn Trait> , ie it is object safe
#[async_trait]
pub trait AccountsApi {

    async fn create_account(&mut self, description: &str) -> anyhow::Result<AccountId>;

    async fn get_balance(&mut self, account_id: &AccountId) -> anyhow::Result<GetBalanceResult>;

    async fn deposit(&mut self, account_id: &AccountId, amount: u32) -> anyhow::Result<GetBalanceResult>;

    async fn withdraw(&mut self, account_id: AccountId, amount: u32) -> anyhow::Result<WithdrawResult>;

    #[allow(unused_variables)]
    async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> anyhow::Result<u32> {
        todo!()
    }
}


/// Future version of the service API.
/// Needs trait-variant to be able to make it object-safe
/// See: https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#where-the-gaps-lie
#[trait_variant::make(AccountsApi2: Send)]
pub trait LocalAccountsApi2 {
    async fn create_account(&mut self, description: &str) -> anyhow::Result<AccountId>;

    async fn get_balance(&mut self, account_id: AccountId) -> anyhow::Result<GetBalanceResult>;

    async fn deposit(&mut self, account_id: AccountId, amount: u32) -> anyhow::Result<GetBalanceResult>;

    async fn withdraw(&mut self, account_id: AccountId, amount: u32) -> anyhow::Result<WithdrawResult>;

    // #[allow(unused_variables)]
    // async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> anyhow::Result<u32> {
    // }
}

