use bcrypt;
use dotenv;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fmt::Display;

use crate::{
    constants::{ID, JWT_SECRET},
    users::read_user,
};

#[derive(Debug)]
pub enum AuthError {
    InvalidPassword,
    UserNotFound,
    DecodeError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    email: String,
    password: String
}

pub fn login(auth: Auth) -> Result<Token, AuthError> {
    if let Some(user) = read_user::find_for_auth(&auth.email) {
        match bcrypt::verify(&auth.password, &user.password) {
            Ok(v) => {
                if !v {
                    return Err(AuthError::InvalidPassword);
                }
            }
            Err(e) => return Err(AuthError::DecodeError(e.to_string())),
        }

        Ok(generate_jwt_token(&user.id.to_string()))
    } else {
        Err(AuthError::UserNotFound)
    }
}

#[derive(Serialize, Debug)]
pub struct Token {
    pub token: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

fn generate_jwt_token(id: &str) -> Token {
    let secret = dotenv::var(JWT_SECRET).unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(&secret.into_bytes()).unwrap();

    let mut claims = BTreeMap::new();
    claims.insert(ID.to_string(), id.to_string());

    let token_str = claims.sign_with_key(&key).unwrap();
    Token { token: token_str }
}

pub fn check_jwt_token(token_str: &str, id: &str) -> bool {
    let secret = dotenv::var(JWT_SECRET).unwrap();
    let key: Hmac<Sha256> = Hmac::new_from_slice(&secret.into_bytes()).unwrap();

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    claims.get(ID).unwrap() == id
}
