use crate::models::{AuthUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub fn find(conn: &mut PgConnection, e_mail: &str) -> Option<User> {
    match users
        .select(User::as_select())
        .filter(email.eq(e_mail))
        .first(conn)
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub fn find_for_auth(conn: &mut PgConnection, e_mail: &str) -> Option<AuthUser> {
    match users
        .select(AuthUser::as_select())
        .filter(email.eq(e_mail))
        .first(conn)
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}
