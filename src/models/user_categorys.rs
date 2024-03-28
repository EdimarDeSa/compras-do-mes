use diesel::{Associations, Identifiable, Queryable, Selectable};
use uuid::Uuid;

use crate::models::user::User;
use crate::schema::user_categorys;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_categorys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserCategory {
    id: Uuid,
    name: String,
    user_id: Uuid,
}