use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::default_products;
use crate::models::default_categorys::DefaultCategorys;
use crate::models::unity_types::UnityTypes;


#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(DefaultCategorys))]
#[diesel(belongs_to(UnityTypes))]
#[diesel(table_name = default_products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DefaultProducts {
    pub id: Uuid,
    pub name: String,
    pub default_categorys_id: Uuid,
    pub unity_types_id: Uuid,
    pub image_url: Option<String>,
}


