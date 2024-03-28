use diesel::{Associations, Identifiable, Queryable, Selectable};
use diesel::data_types::Cents;
use uuid::Uuid;

use crate::schema::shopping_list;
use crate::models::user::User;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = shopping_list)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShoppingList {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub final_value: Cents,
    pub unique_itens: i16,
    pub total_itens: i16,
}