use bcrypt;
use dotenv;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::Serialize;
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::users::read_user;
use crate::constants::constants::ID;

#[derive(Debug)]
pub enum AuthError {
    InvalidPassword,
    UserNotFound,
}

pub fn login(email: &str, password: &str) -> Result<Token, AuthError> {
    if let Some(user) = read_user::find_for_auth(email) {
        if !bcrypt::verify(password, &user.password).unwrap() {
            return Err(AuthError::InvalidPassword);
        }

        let token = generate_jwt_token(&user.id.to_string());
        Ok(token)
    } else {
        Err(AuthError::UserNotFound)
    }
}

#[derive(Serialize, Debug)]
pub struct Token {
    pub token: String,
}

fn generate_jwt_token(id: &str) -> Token {
    let secret = dotenv::var("JWT_SECRET").unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(&secret.into_bytes()).unwrap();

    let mut claims = BTreeMap::new();
    claims.insert(ID.to_string(), id.to_string());

    let token_str = claims.sign_with_key(&key).unwrap();
    Token { token: token_str }
}

pub fn check_jwt_token(token_str: &str, id: &str) -> bool {
    let secret = dotenv::var("JWT_SECRET").unwrap();
    let key: Hmac<Sha256> = Hmac::new_from_slice(&secret.into_bytes()).unwrap();

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    claims.get(ID).unwrap() == id
}
