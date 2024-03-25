use bcrypt;
use diesel::insert_into;
use diesel::prelude::*;
use regex::Regex;

use crate::models::{FullUser, NewUser, User};
use crate::schema::users;
use crate::users::read_user;

#[derive(Debug)]
pub enum CreationError {
    InvalidPassword(String),
    InvalidEmail(String),
    UserAlreadyExists(String),
    CommunicationFailed(String),
}

pub fn new(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, CreationError> {
    if !validate_password(&new_user.password) {
        return Err(CreationError::InvalidPassword(
            "Password must have at last:\n\
            One lowercase letter\n\
            One uppercase letter\n\
            One digit\n\
            One special character from [@$!%*?&]\n\
            Minimum length of 8 digits."
                .to_string(),
        ));
    }

    if !validate_email(&new_user.email) {
        return Err(CreationError::InvalidEmail("Invalid email".to_string()));
    }

    if exist_user(conn, &new_user.email) {
        return Err(CreationError::UserAlreadyExists(
            "User already exists.".to_string(),
        ));
    }

    let mut full_user = FullUser::new(new_user);

    full_user.password = bcrypt::hash(&full_user.password, bcrypt::DEFAULT_COST).unwrap();

    match insert_into(users::table)
        .values(&full_user)
        .returning(User::as_returning())
        .get_result(conn)
    {
        Ok(user) => Ok(user),
        Err(_) => Err(CreationError::CommunicationFailed(
            "Failed to insert user.".to_string(),
        )),
    }
}

fn validate_password(password: &str) -> bool {
    if password.is_empty() {
        return false;
    }

    let has_lowercase = Regex::new(r"[a-z]").unwrap();
    let has_uppercase = Regex::new(r"[A-Z]").unwrap();
    let has_number = Regex::new(r"\d").unwrap();
    let has_symbol = Regex::new(r"[@$!%*?&]").unwrap();
    let has_length = Regex::new(r".{8,}").unwrap();

    has_lowercase.is_match(password)
        && has_uppercase.is_match(password)
        && has_number.is_match(password)
        && has_symbol.is_match(password)
        && has_length.is_match(password)
}

fn validate_email(e_mail: &str) -> bool {
    let is_email = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    is_email.is_match(e_mail)
}

fn exist_user(conn: &mut PgConnection, e_mail: &str) -> bool {
    read_user::find(conn, e_mail).is_some()
}
