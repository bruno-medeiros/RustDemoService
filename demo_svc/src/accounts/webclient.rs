use std::str::FromStr;
use crate::accounts::api::{AccountsApi, DepositResult, GetBalanceResult, WithdrawResult};
use crate::accounts::webapp::{CreateAccountParams, CreateAccountResponse, DepositParams, WithdrawParams};
use async_trait::async_trait;
use axum::http::Uri;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::debug;
use tx_model::AccountId;

#[derive(Clone, Debug)]
pub struct AccountsServiceClient {
    client: Client,
    base_url: Uri,
}

impl AccountsServiceClient {
    pub fn new(base_url: &str) -> AccountsServiceClient {
        let client = Client::new();
        AccountsServiceClient {
            client,
            base_url: base_url.parse().unwrap(),
        }
    }

    async fn send_request<T: Serialize, R: DeserializeOwned>(
        &mut self,
        uri_path: &str,
        params: &T,
    ) -> anyhow::Result<R> {
        let base_url = &self.base_url;
        let url: Uri = Uri::from_str(&format!("{}{}", base_url, uri_path))?;
        let body = serde_json::to_string(&params)?;
        debug!("Sending request to {url} with body:\n {body}");
        let res = self
            .client
            .post(url.to_string())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;
        debug!("Received {} request with body:\n {}", status, body);
        let res: R = serde_json::from_str(&body)?;
        Ok(res)
    }
}

#[async_trait]
impl AccountsApi for AccountsServiceClient {
    async fn create_account(&mut self, description: &str) -> anyhow::Result<AccountId> {
        let params = CreateAccountParams {
            description: description.to_string(),
        };

        let res = self
            .send_request::<_, CreateAccountResponse>("accounts", &params)
            .await?;
        Ok(res.id)
    }

    async fn get_balance(&mut self, account_id: &AccountId) -> anyhow::Result<GetBalanceResult> {
        let res = self
            .send_request::<_, GetBalanceResult>("accounts/get_balance", &account_id)
            .await?;
        Ok(res)
    }

    async fn deposit(
        &mut self,
        account_id: &AccountId,
        amount: u32,
    ) -> anyhow::Result<DepositResult> {
        let params = DepositParams {
            account_id: account_id.clone(),
            amount,
        };
        let res = self
            .send_request("accounts/deposit", &params)
            .await?;
        Ok(res)
    }

    async fn withdraw(
        &mut self,
        account_id: &AccountId,
        amount: u32,
    ) -> anyhow::Result<WithdrawResult> {
        let params = WithdrawParams {
            account_id: account_id.clone(),
            amount,
        };
        let res = self
            .send_request("accounts/withdraw/", &params)
            .await?;
        Ok(res)
    }
}
