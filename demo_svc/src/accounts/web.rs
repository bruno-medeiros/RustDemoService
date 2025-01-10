use crate::accounts::api::{AccountsApi, GetBalanceResult};
use crate::AppState;
use axum::extract::State;
use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use tx_model::AccountId;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateAccount {
    description: String,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    // #[serde(with = "uuid::serde::simple")]
    id: Uuid,
}

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccount>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    let result = accounts.create_account(&payload.description).await;
    match result {
        Ok(id) => {
            let response = CreateAccountResponse { id };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn get_balance(
    State(state): State<Arc<AppState>>,
    Json(params): Json<AccountId>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    let result = accounts.get_balance(&params).await;
    match result {
        Ok(GetBalanceResult::Ok(balance)) => {
            (StatusCode::OK, Json(balance)).into_response()
        }
        Ok(GetBalanceResult::AccountNotFound(account_id)) => {
            (StatusCode::NOT_FOUND, Json(account_id)).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}