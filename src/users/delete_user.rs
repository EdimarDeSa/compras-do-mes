use diesel::{prelude::*, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{connection, schema::users::dsl::*, users::read_user};

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

    if read_user::find_with_id(_id).is_none() {
        return Err(DeletionError::UserNotFound);
    }

    conn.transaction::<_, DeletionError, _>(|conn| {
        match diesel::delete(users.filter(id.eq(_id))).execute(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(DeletionError::TransactionError(e.to_string())),
        }
    })
}
