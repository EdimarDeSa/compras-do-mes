use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    connection,
    schema::users::dsl::*,
    models::user::{AuthUser, PartialUser},
};

pub fn find_with_email(e_mail: &str) -> Option<PartialUser> {
    let conn = &mut connection::establish_connection();

    match users
        .select(PartialUser::as_select())
        .filter(email.eq(e_mail))
        .first(conn)
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub fn find_with_id(_id: &Uuid) -> Option<PartialUser> {
    let conn = &mut connection::establish_connection();

    match users
        .select(PartialUser::as_select())
        .filter(id.eq(_id))
        .first(conn)
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub fn find_for_auth(e_mail: &str) -> Option<AuthUser> {
    let conn = &mut connection::establish_connection();

    match users
        .select(AuthUser::as_select())
        .filter(email.eq(e_mail))
        .first(conn)
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}
