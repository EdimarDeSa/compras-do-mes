use std::sync::{Mutex};
use axum_test::http::{header, StatusCode};
use axum_test::TestServer;
use serde_json::{json};
use crate::auth::jwt_auth::Token;

use crate::auth::urls::create_routes;

static JWT_INFO: Mutex<Option<Token>> = Mutex::new(None);

static URL:&str = "/login";

fn setup_test_server() -> TestServer {
    TestServer::new(create_routes()).unwrap()
}

#[tokio::test]
async fn test_authentication_success() {
    let auth = json!({
        "email": "example@example.com",
        "password": "P@ssword123",
    });
    let response = setup_test_server().post(URL).json(&auth).await;

    assert_eq!(response.status_code(), 200);

    *JWT_INFO.lock().unwrap() = Some(response.json::<Token>());
}

#[tokio::test]
async fn test_authentication_failure() {
    let auth = json!({
        "email": "example@example.com",
        "password": "password",
    });
    let response = setup_test_server().post(URL).json(&auth).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_authentication_with_jwt() {
    let token = JWT_INFO.lock().unwrap().clone().unwrap();

    let json_ = json!({"id": token.id});

    let url = String::from_iter([URL, "/jwt"]);

    let response = setup_test_server().post(&url)
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token.token).parse().unwrap())
        .json(&json_)
        .await;

    response.assert_status(StatusCode::ACCEPTED)
}