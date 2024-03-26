use axum::http::{HeaderMap, StatusCode};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::jwt_auth::*;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    msg: String,
}

async fn post_login(auth: Json<Auth>) -> (StatusCode, Json<Value>) {
    match login(auth.0) {
        Ok(token) => (StatusCode::OK, Json(json!(token))),
        Err(e) => match e {
            AuthError::InvalidPassword => {
                let err = ErrorResponse {
                    error: "InvalidPassword".to_string(),
                    msg: "Invalid password".to_string(),
                };
                (StatusCode::BAD_REQUEST, Json(json!(err)))
            }
            AuthError::UserNotFound => {
                let err = ErrorResponse {
                    error: "UserNotFound".to_string(),
                    msg: "User not found".to_string(),
                };
                (StatusCode::NOT_FOUND, Json(json!(err)))
            }
            AuthError::DecodeError(e) => {
                let err = ErrorResponse {
                    error: "DecodeError".to_string(),
                    msg: e,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(err)))
            }
        },
    }
}

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    id: String,
}

async fn get_login(header_map: HeaderMap, json: Json<JsonResponse>) -> StatusCode {
    let token = match header_map.get("Authorization") {
        Some(auth) => {
            let auth = auth.to_str().unwrap();
            auth.split_whitespace().nth(1).unwrap()
        }
        None => return StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
    };

    if !check_jwt_token(token, &json.id) {
        return StatusCode::UNAUTHORIZED;
    };

    StatusCode::ACCEPTED
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/login", post(post_login))
        .route("/login/jwt", post(get_login))
}
