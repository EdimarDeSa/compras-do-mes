use crate::auth;
use crate::auth::check_jwt_token;
use crate::constants::*;
use crate::users::{user_models::*, *};
use chrono::NaiveDate;
use std::collections::HashMap;

const INITIAL_EMAIL: &str = "email_super.criativo@queemail.com";
const NEW_EMAIL: &str = "email@khyyuyu.com";
const INITIAL_PASSWORD: &str = "Senh@Mu1toF0rt3";
const INITIAL_NAME: &str = "Fulanisson de Teste";
const NEW_NAME: &str = "Fulanisson de Teste Alterado";

#[test]
fn test_1_create_user_success() {
    let new_user = NewUser {
        nickname: INITIAL_NAME.to_string(),
        email: INITIAL_EMAIL.to_string(),
        password: INITIAL_PASSWORD.to_string(),
        birth_date: Some(NaiveDate::parse_from_str("1995-09-04", BIRTH_DATE_FORMAT).unwrap()),
    };

    assert!(create_user::new(&new_user).is_ok())
}

#[test]
fn test_2_create_user_fails_in_password() {
    let new_user = NewUser {
        nickname: INITIAL_NAME.to_string(),
        email: INITIAL_EMAIL.to_string(),
        password: "<PASSWORD>".to_string(),
        birth_date: Some(NaiveDate::parse_from_str("1995-09-04", BIRTH_DATE_FORMAT).unwrap()),
    };

    assert!(create_user::new(&new_user).is_err())
}

#[test]
fn test_3_read_user() {
    let email = INITIAL_EMAIL.to_string();

    let user = read_user::find_with_email(&email).unwrap();

    assert_eq!(user.nickname, INITIAL_NAME.to_string());
    assert_eq!(user.email, email);
    assert_eq!(
        user.birth_date,
        Some(NaiveDate::parse_from_str("1995-09-04", BIRTH_DATE_FORMAT).unwrap())
    );
}

#[test]
fn test_4_update_user() {
    let user = read_user::find_with_email(INITIAL_EMAIL).unwrap();

    let mut changes = HashMap::new();
    changes.insert("nickname".to_string(), NEW_NAME.to_string());
    changes.insert("email".to_string(), NEW_EMAIL.to_string());

    let alter_user = AlterUser {
        id: user.id,
        changes: changes.clone(),
    };

    let altered = update_user::update(alter_user).unwrap();

    assert_eq!(&*altered.nickname.unwrap(), NEW_NAME.to_string());
    assert_eq!(&*altered.email.unwrap(), NEW_EMAIL.to_string());
}

#[test]
fn test_5_login_success() {
    let token = auth::login(NEW_EMAIL, INITIAL_PASSWORD);
    assert!(token.is_ok())
}

#[test]
fn test_6_login_fails() {
    let token = auth::login(NEW_EMAIL, "INITIAL_PASSWORD");
    assert!(token.is_err())
}

#[test]
fn test_7_check_jwt() {
    let token = auth::login(NEW_EMAIL, INITIAL_PASSWORD).unwrap();
    if let Some(user) = read_user::find_with_email(NEW_EMAIL) {
        assert!(check_jwt_token(&token.token, &user.id.to_string()))
    }
}

#[test]
fn test_99_delete_user_success() {
    let email = NEW_EMAIL.to_string();
    let id = read_user::find_with_email(&email).unwrap().id;

    assert!(delete_user::remove(&id).is_ok())
}
