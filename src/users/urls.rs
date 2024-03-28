use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::users::{
    read_user,
    {create_user, create_user::CreationError},
    {delete_user, delete_user::DeletionError},
    {update_user, update_user::UpdateError},
};
use crate::models::user::{AlterUser, NewUser};

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
            }
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

async fn get_user_with_email(email: Path<String>) -> (StatusCode, Json<Value>) {
    match read_user::find_with_email(&email) {
        Some(u) => (StatusCode::FOUND, Json(json!(u))),
        None => {
            let msg = json!(ErrorResponse {
                error: "UserNotFound".to_string(),
                msg: "User not found".to_string()
            });
            (StatusCode::NOT_FOUND, Json(msg))
        }
    }
}

async fn get_user_with_id(id: Path<Uuid>) -> (StatusCode, Json<Value>) {
    match read_user::find_with_id(&id) {
        Some(u) => (StatusCode::FOUND, Json(json!(u))),
        None => {
            let msg = json!(ErrorResponse {
                error: "UserNotFound".to_string(),
                msg: "User not found".to_string()
            });
            (StatusCode::NOT_FOUND, Json(msg))
        }
    }
}

async fn update(alter_user: Json<AlterUser>) -> (StatusCode, Json<Value>) {
    match update_user::update(&alter_user) {
        Ok(u) => (StatusCode::OK, Json(json!(u))),
        Err(err) => match err {
            UpdateError::TransactionError(e) => {
                let msg = ErrorResponse {
                    error: "TransactionError".to_string(),
                    msg: e
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(msg)))
            },
            UpdateError::DateFormatError(e) => {
                let msg = ErrorResponse {
                    error: "DateFormatError".to_string(),
                    msg: e
                };
                (StatusCode::BAD_REQUEST, Json(json!(msg)))
            },
            UpdateError::InvalidPassword(e) => {
                let msg = ErrorResponse {
                    error: "InvalidPassword".to_string(),
                    msg: e
                };
                (StatusCode::BAD_REQUEST, Json(json!(msg)))
            },
            UpdateError::InvalidEmail(e) => {
                let msg = ErrorResponse {
                    error: "InvalidEmail".to_string(),
                    msg: e
                };
                (StatusCode::BAD_REQUEST, Json(json!(msg)))
            },
            UpdateError::InvalidFieldError(e) => {
                let msg = ErrorResponse {
                    error: "InvalidFieldError".to_string(),
                    msg: e
                };
                (StatusCode::BAD_REQUEST, Json(json!(msg)))
            },
            UpdateError::EncryptionError(e) => {
                let msg = ErrorResponse {
                    error: "EncryptionError".to_string(),
                    msg: e
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(msg)))
            },
            UpdateError::UserNotFound => {
                let msg = ErrorResponse {
                    error: "UserNotFound".to_string(),
                    msg: "User not found".to_string()
                };
                (StatusCode::NOT_FOUND, Json(json!(msg)))
            },
        }
    }
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/delete/:user_id", delete(remove))
        .route("/search/email/:email", get(get_user_with_email))
        .route("/search/id/:id", get(get_user_with_id))
        .route("/update", put(update))
}
