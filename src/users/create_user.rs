use crate::connection;
use bcrypt;
use diesel::insert_into;
use diesel::prelude::*;
use regex::Regex;

use crate::schema::users;
use crate::users::read_user;
use crate::users::user_models::{FullUser, NewUser, User};

#[derive(Debug)]
pub enum CreationError {
    InvalidPassword(String),
    InvalidEmail,
    UserAlreadyExists,
    TransactionError(String),
    EncryptionError(String),
    RegexError(String),
}

impl From<diesel::result::Error> for CreationError {
    fn from(error: diesel::result::Error) -> Self {
        CreationError::TransactionError(error.to_string())
    }
}

pub fn new(new_user: &NewUser) -> Result<User, CreationError> {
    if let Err(e) = validate_user_data(&new_user) {
        return Err(e);
    }

    let full_user = match create_full_user(new_user) {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    let conn = &mut connection::establish_connection();
    conn.transaction::<_, CreationError, _>(|conn| {
        match insert_into(users::table)
            .values(full_user)
            .returning(User::as_returning())
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(e) => Err(CreationError::TransactionError(e.to_string())),
        }
    })
}

fn validate_user_data(new_user: &NewUser) -> Result<(), CreationError> {
    if let Err(e) = validate_password(&new_user.password) {
        return Err(e);
    }

    if let Err(e) = validate_email(&new_user.email) {
        return Err(e);
    }

    if let Err(e) = exist_user(&new_user.email) {
        return Err(e);
    }
}

fn validate_password(password: &str) -> Result<(), CreationError> {
    if password.is_empty() {
        return Err(CreationError::InvalidPassword(
            "Password cannot be empty!".to_string(),
        ));
    }

    match Regex::new(r"[a-z]") {
        Ok(r) => {
            if !r.is_match(password) {
                return Err(CreationError::InvalidPassword(
                    "Password must be at last one lowercase letter".to_string(),
                ));
            }
        }
        Err(e) => return Err(CreationError::RegexError(e.to_string())),
    };

    match Regex::new(r"[A-Z]") {
        Ok(r) => {
            if !r.is_match(password) {
                return Err(CreationError::InvalidPassword(
                    "Password must be at last one uppercase letter".to_string(),
                ));
            }
        }
        Err(e) => return Err(CreationError::RegexError(e.to_string())),
    };
    match Regex::new(r"\d") {
        Ok(r) => {
            if !r.is_match(password) {
                return Err(CreationError::InvalidPassword(
                    "Password must be at last one digit".to_string(),
                ));
            }
        }
        Err(e) => return Err(CreationError::RegexError(e.to_string())),
    };
    match Regex::new(r"[@$!%*?&]") {
        Ok(r) => {
            if !r.is_match(password) {
                return Err(CreationError::InvalidPassword(
                    "Password must be at last one special character -> (@$!%*?&)".to_string(),
                ));
            }
        }
        Err(e) => return Err(CreationError::RegexError(e.to_string())),
    };
    match Regex::new(r".{8,}") {
        Ok(r) => {
            if !r.is_match(password) {
                return Err(CreationError::InvalidPassword(
                    "Password must be at least 8 characters long".to_string(),
                ));
            }
        }
        Err(e) => return Err(CreationError::RegexError(e.to_string())),
    };

    Ok(())
}

fn validate_email(e_mail: &str) -> Result<(), CreationError> {
    match Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
        Ok(r) => {
            if !r.is_match(e_mail) {
                return Err(CreationError::InvalidEmail);
            }
        }
        Err(e) => return Err(CreationError::RegexError(e.to_string())),
    };

    Ok(())
}

fn exist_user(e_mail: &str) -> Result<(), CreationError> {
    if read_user::find(e_mail).is_some() {
        return Err(CreationError::UserAlreadyExists);
    }
    Ok(())
}

fn create_full_user(new_user: &NewUser) -> Result<FullUser, CreationError> {
    let mut full_user = FullUser::new(new_user);

    match bcrypt::hash(&full_user.password, bcrypt::DEFAULT_COST) {
        Ok(hashed_password) => {
            full_user.password = hashed_password;
            Ok(full_user)
        },
        Err(e) => return Err(CreationError::EncryptionError(e.to_string())),
    }
}