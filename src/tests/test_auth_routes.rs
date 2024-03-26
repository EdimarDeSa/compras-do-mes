use std::sync::Mutex;
use axum_test::TestServer;
use serde_json::json;
use crate::auth::jwt_auth::Token;

use crate::auth::urls::create_routes;

static JWT_INFO: Mutex<Option<String>> = Mutex::new(None);


#[tokio::test]
async fn test_authentication_success() {

    let url = "/login";
    let auth = json!({
        "email": "example@example.com",
        "password": "P@ssword123",
    });

    let server = TestServer::new(create_routes()).unwrap();
    let response = server.post(url).json(&auth).await;


    assert_eq!(response.status_code(), 200);

    *JWT_INFO.lock().unwrap() = Some(response.json::<Token>().token);
}