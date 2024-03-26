use chrono::NaiveDate;
use diesel::prelude::*;

use crate::{
    connection,
    constants::BIRTH_DATE_FORMAT,
    schema::users::dsl::*,
    users::{read_user, user_models::AlterUser},
    validators,
};

#[derive(Debug)]
pub enum UpdateError {
    UserNotFound,
    TransactionError(String),
    DateFormatError(String),
    InvalidEmail(String),
    InvalidPassword(String),
    EncryptionError(String),
    InvalidFieldError(String),
}

impl From<diesel::result::Error> for UpdateError {
    fn from(error: diesel::result::Error) -> Self {
        UpdateError::TransactionError(error.to_string())
    }
}

#[derive(Debug, Default)]
pub struct UpdateInfo {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub birth_date: Option<String>,
}

pub fn update(new_user: AlterUser) -> Result<UpdateInfo, UpdateError> {
    if read_user::find_with_id(&new_user.id).is_none() {
        return Err(UpdateError::UserNotFound);
    };

    let mut update_info = UpdateInfo::default();

    let conn = &mut connection::establish_connection();

    conn.transaction::<_, UpdateError, _>(|conn| {
        for (field, value) in new_user.changes {
            match field.to_lowercase().as_str() {
                "nickname" => {
                    diesel::update(users.filter(id.eq(&new_user.id)))
                        .set(nickname.eq(&value))
                        .execute(conn)
                        .map_err(|e| UpdateError::TransactionError(e.to_string()))?;
                    update_info.nickname = Some(value);
                }

                "email" => {
                    if let Err(e) = validators::validate_email(&value) {
                        return Err(UpdateError::InvalidEmail(e.to_string()));
                    }

                    diesel::update(users.filter(id.eq(&new_user.id)))
                        .set(email.eq(&value))
                        .execute(conn)
                        .map_err(|e| UpdateError::TransactionError(e.to_string()))?;
                    update_info.email = Some(value);
                }

                "password" => {
                    if let Err(e) = validators::validate_password(&value) {
                        return Err(UpdateError::InvalidPassword(e.to_string()));
                    }

                    let hashed_password = match bcrypt::hash(&value, bcrypt::DEFAULT_COST) {
                        Ok(hashed_password) => hashed_password,
                        Err(e) => return Err(UpdateError::EncryptionError(e.to_string())),
                    };

                    diesel::update(users.filter(id.eq(&new_user.id)))
                        .set(password.eq(&hashed_password))
                        .execute(conn)
                        .map_err(|e| UpdateError::TransactionError(e.to_string()))?;
                    update_info.password = Some(value);
                }

                "birth_date" => {
                    if let Ok(date) = NaiveDate::parse_from_str(&value, BIRTH_DATE_FORMAT) {
                        diesel::update(users.filter(id.eq(&new_user.id)))
                            .set(birth_date.eq(date))
                            .execute(conn)
                            .map_err(|e| UpdateError::TransactionError(e.to_string()))?;
                        update_info.birth_date = Some(value);
                    } else {
                        return Err(UpdateError::DateFormatError(format!(
                            "Date format must be {}",
                            BIRTH_DATE_FORMAT
                        )));
                    };
                }

                _ => {
                    return Err(UpdateError::InvalidFieldError(format!(
                        "Invalid key {}",
                        field
                    )))
                }
            }
        }

        Ok(update_info)
    })
}
