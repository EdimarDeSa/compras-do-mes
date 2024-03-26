use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::Serialize;
use serde_json::{json, Value};

use compras_do_mes::users;
use compras_do_mes::users::create_user::CreationError;
use compras_do_mes::users::user_models::NewUser;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    msg: String,
}


async fn create_user(new_user: Json<NewUser>) -> (StatusCode, Json<Value>) {
    match users::create_user::new(&new_user) {
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new().route("/create_user", post(create_user));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
