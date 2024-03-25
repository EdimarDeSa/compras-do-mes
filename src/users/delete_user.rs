use crate::connection;
use crate::schema::users::dsl::*;
use crate::users::read_user;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Debug)]
pub enum DeletionError {
    UserNotFound,
    TransactionError(String),
}

impl From<diesel::result::Error> for DeletionError {
    fn from(error: diesel::result::Error) -> Self {
        DeletionError::TransactionError(error.to_string())
    }
}

pub fn remove(_id: &Uuid) -> Result<usize, DeletionError> {
    let conn = &mut connection::establish_connection();

    conn.transaction::<_, DeletionError, _>(|conn| {
        if read_user::find_with_id(_id).is_none() {
            return Err(DeletionError::UserNotFound);
        }

        match diesel::delete(users.filter(id.eq(_id))).execute(conn) {
            Ok(q) => Ok(q),
            Err(e) => Err(DeletionError::TransactionError(e.to_string())),
        }
    })
}
