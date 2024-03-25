use crate::connection;
use crate::schema::users::dsl::*;
use crate::users::read_user;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;
use diesel::prelude::*;

#[derive(Debug)]
pub enum DeletionError {
    UserNotFound(String),
    CommunicationFailed(String),
}

pub fn remove(_id: &Uuid) -> Result<usize, DeletionError> {
    let conn = &mut connection::establish_connection();

    if read_user::find_for_delete(_id).is_none() {
        return Err(DeletionError::UserNotFound(format!(
            "Não foi possível encontrar o usuário com  id: {}", _id.to_string(),
        )));
    }

    match diesel::delete(users.filter(id.eq(_id))).execute(conn) {
        Ok(q) => Ok(q),
        Err(e) => Err(DeletionError::CommunicationFailed(e.to_string())),
    }
}
