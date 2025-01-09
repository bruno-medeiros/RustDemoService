use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::accounts::api::AccountsApi;
use crate::AppState;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateAccount {
    description: String,
}


#[derive(Serialize)]
pub struct CreateAccountResponse {
    username: String,
}

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccount>,
) -> (StatusCode, Json<CreateAccountResponse>) {
    let mut accounts = state.accounts.lock().await;

    // TODO:

    match accounts.create_account(&payload.description).await {
        Ok(_) => {}
        Err(_) => {}
    };

    let response = CreateAccountResponse {
        username: payload.description,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(response))
}