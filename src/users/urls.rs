use axum::{http::StatusCode, routing::{post, delete}, Json, Router, extract::Path};
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::users::{create_user, create_user::CreationError, user_models::NewUser};
use crate::users::{delete_user, delete_user::DeletionError};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    msg: String,
}

async fn create(new_user: Json<NewUser>) -> (StatusCode, Json<Value>) {
    match create_user::new(&new_user) {
        Ok(u) => (StatusCode::CREATED, Json(json!(u))),
        Err(e) => match e {
            CreationError::UserAlreadyExists => {
                let err = ErrorResponse {
                    error: "UserAlreadyExists".to_string(),
                    msg: "User already exists".to_string(),
                };
                (StatusCode::CONFLICT, Json(json!(err)))
            }
            CreationError::InvalidEmail(e) => {
                let err = ErrorResponse {
                    error: "InvalidEmail".to_string(),
                    msg: e,
                };
                (StatusCode::BAD_REQUEST, Json(json!(err)))
            }
            CreationError::InvalidPassword(e) => {
                let err = ErrorResponse {
                    error: "InvalidPassword".to_string(),
                    msg: e,
                };
                (StatusCode::BAD_REQUEST, Json(json!(err)))
            }
            CreationError::EncryptionError(e) => {
                let err = ErrorResponse {
                    error: "EncryptionError".to_string(),
                    msg: e,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(err)))
            }
            CreationError::TransactionError(e) => {
                let err = ErrorResponse {
                    error: "TransactionError".to_string(),
                    msg: e,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(err)))
            }
        },
    }
}

async fn remove(user_id: Path<Uuid>) -> (StatusCode, Json<Value>) {
    match delete_user::remove(&user_id) {
        Ok(u) => (StatusCode::OK, Json(json!({"removals": u}))),
        Err(e) => match e {
            DeletionError::UserNotFound => {
                let err = ErrorResponse {
                    error: "UserNotFound".to_string(),
                    msg: "User not found".to_string(),
                };
                (StatusCode::NOT_FOUND, Json(json!(err)))
            },
            DeletionError::TransactionError(e) => {
                let err = ErrorResponse {
                    error: "TransactionError".to_string(),
                    msg: e,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(err)))
            }
        },
    }
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/delete/:user_id", delete(remove))
}
