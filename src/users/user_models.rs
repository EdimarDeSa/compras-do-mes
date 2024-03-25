use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FullUser {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub birth_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

impl FullUser {
    pub fn new(new_user: &NewUser) -> Self {
        let id = Uuid::new_v4();

        let created_at = Utc::now().naive_utc();

        Self {
            id,
            nickname: new_user.nickname.clone(),
            email: new_user.email.clone(),
            password: new_user.password.clone(),
            birth_date: new_user.birth_date,
            created_at,
        }
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub birth_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct NewUser {
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUser {
    pub id: Uuid,
    pub password: String,
}
