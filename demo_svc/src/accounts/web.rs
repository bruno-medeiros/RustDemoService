use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}


#[derive(Serialize)]
pub struct CreateAccountResponse {
    username: String,
}

pub async fn create_account(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<CreateAccountResponse>) {
    let user = CreateAccountResponse {
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}