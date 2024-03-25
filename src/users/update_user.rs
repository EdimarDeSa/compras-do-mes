use diesel::prelude::*;

use crate::connection;
use crate::schema::users::dsl::*;
use crate::users::read_user;
use crate::users::user_models::AlterUser;

#[derive(Debug)]
pub enum UpdateError {
    UserNotFound(String),
    CommunicationFailed(String),
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
        return Err(UpdateError::UserNotFound(format!(
            "Não foi possível encontrar o usuário com  id: {}",
            &new_user.id.to_string(),
        )));
    };

    let mut update_info = UpdateInfo::default();

    let conn = &mut connection::establish_connection();

    if let Some(nick) = new_user.nickname {
        match diesel::update(users.filter(id.eq(&new_user.id)))
            .set(nickname.eq(&nick))
            .returning(nickname)
            .execute(conn)
        {
            Ok(_) => update_info.nickname = Some(nick),
            Err(e) => return Err(UpdateError::CommunicationFailed(e.to_string())),
        };
    };

    if let Some(e_mail) = new_user.email {
        match diesel::update(users.filter(id.eq(&new_user.id)))
            .set(email.eq(&e_mail))
            .returning(email)
            .execute(conn)
        {
            Ok(_) => update_info.email = Some(e_mail),
            Err(e) => return Err(UpdateError::CommunicationFailed(e.to_string())),
        };
    }

    if let Some(pass) = new_user.password {
        match diesel::update(users.filter(id.eq(&new_user.id)))
            .set(password.eq(&pass))
            .returning(password)
            .execute(conn)
        {
            Ok(_) => update_info.password = Some(pass),
            Err(e) => return Err(UpdateError::CommunicationFailed(e.to_string())),
        };
    }

    if let Some(birthdate) = new_user.birth_date {
        match diesel::update(users.filter(id.eq(&new_user.id)))
            .set(birth_date.eq(&birthdate))
            .returning(birth_date)
            .execute(conn)
        {
            Ok(_) => update_info.birth_date = Some(birthdate.to_string()),
            Err(e) => return Err(UpdateError::CommunicationFailed(e.to_string())),
        };
    }

    Ok(update_info)
}
