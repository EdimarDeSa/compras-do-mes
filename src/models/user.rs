use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub birth_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn new(new_user: &NewUser) -> Self {
        Self {
            nickname: new_user.nickname.clone(),
            email: new_user.email.clone(),
            password: new_user.password.clone(),
            birth_date: new_user.birth_date,
            ..Default::default()
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            nickname: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            birth_date: None,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PartialUser {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub birth_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlterUser {
    pub id: Uuid,
    pub changes: HashMap<String, String>,
}
