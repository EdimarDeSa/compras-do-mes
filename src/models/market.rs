use diesel::{Associations, Identifiable, Queryable, Selectable};
use uuid::Uuid;

use crate::models::user::User;
use crate::schema::market;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = market)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Market {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
}