use chrono::NaiveDate;
use diesel::prelude::*;

use crate::connection;
use crate::constants::BIRTH_DATE_FORMAT;
use crate::schema::users::dsl::*;
use crate::users::read_user;
use crate::users::user_models::AlterUser;

#[derive(Debug)]
pub enum UpdateError {
    UserNotFound,
    TransactionError(String),
    DateFormatError(String),
}

impl From<diesel::result::Error> for UpdateError {
    fn from(error: diesel::result::Error) -> Self {
        UpdateError::TransactionError(error.to_string())
    }
}

#[derive(Debug)]
pub struct UpdateInfo {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub birth_date: Option<String>,
}

impl Default for UpdateInfo {
    fn default() -> Self {
        Self {
            nickname: None,
            email: None,
            password: None,
            birth_date: None,
        }
    }
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
                    diesel::update(users.filter(id.eq(&new_user.id)))
                        .set(email.eq(&value))
                        .execute(conn)
                        .map_err(|e| UpdateError::TransactionError(e.to_string()))?;
                    update_info.email = Some(value);
                }
                "password" => {
                    diesel::update(users.filter(id.eq(&new_user.id)))
                        .set(password.eq(&value))
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
                _ => (),
            }
        }

        Ok(update_info)
    })
}
