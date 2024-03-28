use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::unity_types;


#[derive(Debug, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = unity_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UnityTypes {
    pub id: Uuid,
    pub name: String,
    pub calc_value: i16,
}
