use axum::{Json, Router, routing::{get, post}};
use axum::http::{header, HeaderMap, StatusCode, Response};
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::{json, Value};

use super::jwt_auth::*;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    msg: String,
}

async fn post_login(auth: Json<Auth>) -> (StatusCode, Json<Value>) {
    match login(auth.0) {
        Ok(token) => {
            let body = Json(json!(token));
            let mut response = Response::new(body);
            response.headers_mut().insert(
                header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            );
            (StatusCode::OK, response.into_body())
        },
        Err(e) => {
            match e {
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
            }
        }
    }
}

async fn get_login(header_map: HeaderMap) -> impl IntoResponse {
    println!("{:?}", header_map);
    // check_jwt_token()
    StatusCode::OK
}

pub fn create_routes() -> Router {
    let routes = Router::new()
        .route("/login", post(post_login))
        .route("/login", get(get_login));

    routes
}