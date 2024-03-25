use crate::connection;
use crate::schema::users::dsl::*;
use crate::users::user_models::{AuthUser, User};
use diesel::prelude::*;
use uuid::Uuid;

pub fn find(e_mail: &str) -> Option<User> {
    let conn = &mut connection::establish_connection();

    match users
        .select(User::as_select())
        .filter(email.eq(e_mail))
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

pub fn find_for_delete(_id: &Uuid) -> Option<User> {
    let conn = &mut connection::establish_connection();

    match users
        .select(User::as_select())
        .filter(id.eq(_id))
        .first(conn)
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}
