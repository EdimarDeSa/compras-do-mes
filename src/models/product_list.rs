use diesel::{Associations, Identifiable, Queryable, Selectable};
use diesel::data_types::Cents;
use uuid::Uuid;

use crate::models::shopping_list::ShoppingList;
use crate::models::user_products::UserProducts;
use crate::schema::product_list;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(ShoppingList))]
#[diesel(belongs_to(UserProducts, foreign_key=user_product_id))]
#[diesel(table_name = product_list)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductList {
    pub id: Uuid,
    pub shopping_list_id: Uuid,
    pub user_product_id: Uuid,
    pub quantity: f32,
    pub value: Cents,
    pub total: Cents,
    pub on_cart: Option<bool>,
}