use std::sync::Mutex;
use axum_test::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;

use crate::users::urls::create_routes;
use crate::users::user_models::{NewUser, User};

fn setup_test_server() -> TestServer {
    TestServer::new(create_routes()).unwrap()
}

static LITE_DB: Mutex<Option<User>> = Mutex::new(None);

#[tokio::test]
async fn test_create_user_success_without_birth_date() {
    let new_user = NewUser {
        nickname: "Testulino da Silva Testes".to_string(),
        email: "email_super_criativo_pakas@email.com".to_string(),
        password: "S3nh@Mu1t0Gr@nd3".to_string(),
        birth_date: None
    };

    let response = setup_test_server()
        .post("/create")
        .json(&json!(new_user))
        .await;

    response.assert_status(StatusCode::CREATED);

    *LITE_DB.lock().unwrap() = Some(response.json::<User>())
}

#[tokio::test]
async fn test_delete_user() {
    let id = LITE_DB.lock().unwrap().clone().unwrap().id;

    let response = setup_test_server().delete(&format!("/delete/{id}")).await;

    response.assert_status_ok();
    response.assert_json(&json!({"removals": 1}));
}