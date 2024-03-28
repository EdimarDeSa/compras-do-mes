use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::default_categorys;


#[derive(Debug, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = default_categorys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DefaultCategorys {
    pub id: Uuid,
    pub name: String,
}
