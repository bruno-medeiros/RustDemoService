use std::todo;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Balance = u32;
pub type AccountId = Uuid;

pub struct Account {
    pub id: AccountId,
    pub description: String,
    pub balance: Balance,
    pub points: Balance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateAccountParams {
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    // #[serde(with = "uuid::serde::simple")]
    pub id: AccountId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositParams {
    pub account_id: AccountId,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetBalanceResult {
    Ok(Balance),
    AccountNotFound(AccountId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepositResult {
    Ok(Balance),
    AccountNotFound(AccountId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawParams {
    pub account_id: AccountId,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WithdrawResult {
    Ok(Balance),
    AccountNotFound(AccountId),
    NotEnoughBalance(Balance),
}

/// Alternative to `AccountsApi` using async-trait
/// The main advantage is that it allows Box<dyn Trait> , ie it is object safe
#[async_trait]
pub trait AccountsApi {
    async fn create_account(&mut self, description: &str) -> anyhow::Result<CreateAccountResponse>;

    async fn get_balance(&mut self, account_id: &AccountId) -> anyhow::Result<GetBalanceResult>;

    async fn deposit(
        &mut self,
        account_id: &AccountId,
        amount: u32,
    ) -> anyhow::Result<DepositResult>;

    async fn withdraw(
        &mut self,
        account_id: &AccountId,
        amount: u32,
    ) -> anyhow::Result<WithdrawResult>;

    #[allow(unused_variables)]
    async fn transfer(
        &mut self,
        from: AccountId,
        balance: u32,
        to: AccountId,
    ) -> anyhow::Result<u32> {
        todo!()
    }
}

/// Upcoming version of the service API.
/// Needs trait-variant to be able to make it object-safe
/// See: https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#where-the-gaps-lie
#[trait_variant::make(AccountsApi2: Send)]
pub trait LocalAccountsApi2 {
    async fn create_account(&mut self, description: &str) -> anyhow::Result<AccountId>;

    async fn get_balance(&mut self, account_id: AccountId) -> anyhow::Result<GetBalanceResult>;

    async fn deposit(
        &mut self,
        account_id: AccountId,
        amount: u32,
    ) -> anyhow::Result<GetBalanceResult>;

    async fn withdraw(
        &mut self,
        account_id: AccountId,
        amount: u32,
    ) -> anyhow::Result<WithdrawResult>;

    // #[allow(unused_variables)]
    // async fn transfer(&mut self, from: AccountId, balance: u32, to: AccountId) -> anyhow::Result<u32> {
    // }
}
