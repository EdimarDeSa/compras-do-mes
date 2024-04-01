use std::collections::HashMap;
use axum_test::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;
use std::sync::Mutex;

use crate::users::urls::create_routes;
use crate::models::user::{NewUser, User};

fn setup_test_server() -> TestServer {
    TestServer::new(create_routes()).unwrap()
}

static LITE_DB: Mutex<Option<User>> = Mutex::new(None);

#[tokio::test]
async fn test_1_create_user_success_without_birth_date() {
    let new_user = NewUser {
        nickname: "Testulino da Silva Testes".to_string(),
        email: "email_super_criativo_pakas@email.com".to_string(),
        password: "S3nh@Mu1t0Gr@nd3".to_string(),
        birth_date: None,
    };

    let response = setup_test_server()
        .post("/create")
        .json(&json!(new_user))
        .await;

    response.assert_status(StatusCode::CREATED);

    *LITE_DB.lock().unwrap() = Some(response.json::<User>())
}

#[tokio::test]
async fn test_2_get_user_with_email() {
    let user = LITE_DB.lock().unwrap().clone().unwrap();

    let response = setup_test_server()
        .get(&format!("/search/email/{}", user.email))
        .await;

    response.assert_status(StatusCode::FOUND);
    response.assert_json(&json!(user))
}

#[tokio::test]
async fn test_3_get_user_with_id() {
    let user = LITE_DB.lock().unwrap().clone().unwrap();

    let response = setup_test_server()
        .get(&format!("/search/id/{}", user.id))
        .await;

    response.assert_status(StatusCode::FOUND);
    response.assert_json(&json!(user))
}

#[tokio::test]
async fn test_4_update_user() {
    let user = LITE_DB.lock().unwrap().clone().unwrap();

    let new_nick = "Testulino da Silva Testes dos testantes".to_string();

    let mut changes = HashMap::new();
    changes.insert("nickname".to_string(), Some(new_nick.clone()));
    changes.insert("birth_date".to_string(), None);
    changes.insert("email".to_string(), None);
    changes.insert("password".to_string(), None);

    let response = setup_test_server()
        .put("/update")
        .json(&json!({
        "id": user.id,
        "changes": {"nickname": new_nick}
    }))
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_json(&json!(changes))
}

#[tokio::test]
async fn test_5_delete_user() {
    let id = LITE_DB.lock().unwrap().clone().unwrap().id;

    let response = setup_test_server().delete(&format!("/delete/{id}")).await;

    response.assert_status_ok();
    response.assert_json(&json!({"removals": 1}));
}
