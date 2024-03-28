use diesel::{Associations, Identifiable, Queryable, Selectable};
use diesel::data_types::Cents;
use uuid::Uuid;

use crate::models::unity_types::UnityTypes;
use crate::models::user_categorys::UserCategory;
use crate::models::user::User;
use crate::schema::user_products;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(UnityTypes, foreign_key=unity_type_id))]
#[diesel(belongs_to(UserCategory, foreign_key=category_id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProducts {
    id: Uuid,
    name: String,
    unity_type_id: Uuid,
    value: Cents,
    value_unity_type_id: Uuid,
    category_id: Uuid,
    notes: Option<String>,
    barcode: Option<String>,
    image_url: Option<String>,
    user_id: Uuid,
}