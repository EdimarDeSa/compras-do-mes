use bcrypt;
use diesel::{insert_into, prelude::*};

use crate::{
    connection,
    schema::users,
    users::read_user,
    models::user::{User, NewUser, PartialUser},
    validators::{validate_email, validate_password},
};

#[derive(Debug)]
pub enum CreationError {
    InvalidPassword(String),
    InvalidEmail(String),
    UserAlreadyExists,
    TransactionError(String),
    EncryptionError(String),
}

impl From<diesel::result::Error> for CreationError {
    fn from(error: diesel::result::Error) -> Self {
        CreationError::TransactionError(error.to_string())
    }
}

pub fn new(new_user: &NewUser) -> Result<PartialUser, CreationError> {
    validate_user_data(new_user)?;

    let full_user = create_full_user(new_user)?;

    let conn = &mut connection::establish_connection();
    conn.transaction::<_, CreationError, _>(|conn| {
        match insert_into(users::table)
            .values(full_user)
            .returning(PartialUser::as_returning())
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(e) => Err(CreationError::TransactionError(e.to_string())),
        }
    })
}

fn validate_user_data(new_user: &NewUser) -> Result<(), CreationError> {
    if let Err(e) = validate_password(&new_user.password) {
        return Err(CreationError::InvalidPassword(e.to_string()));
    }

    if let Err(e) = validate_email(&new_user.email) {
        return Err(CreationError::InvalidEmail(e.to_string()));
    }

    exist_user(&new_user.email)?;

    Ok(())
}

fn exist_user(e_mail: &str) -> Result<(), CreationError> {
    if read_user::find_with_email(e_mail).is_some() {
        return Err(CreationError::UserAlreadyExists);
    }
    Ok(())
}

fn create_full_user(new_user: &NewUser) -> Result<User, CreationError> {
    let mut full_user = User::new(new_user);

    match bcrypt::hash(&full_user.password, bcrypt::DEFAULT_COST) {
        Ok(hashed_password) => {
            full_user.password = hashed_password;
        }
        Err(e) => return Err(CreationError::EncryptionError(e.to_string())),
    }

    Ok(full_user)
}
