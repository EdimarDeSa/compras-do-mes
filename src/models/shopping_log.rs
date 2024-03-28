use chrono::NaiveDate;
use diesel::{Associations, Identifiable, Queryable, Selectable};
use uuid::Uuid;

use crate::models::shopping_list::ShoppingList;
use crate::models::market::Market;
use crate::models::user::User;
use crate::schema::shopping_log;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(ShoppingList))]
#[diesel(belongs_to(Market))]
#[diesel(table_name = shopping_log)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShoppingLog {
    id: Uuid,
    date: NaiveDate,
    user_id: Uuid,
    shopping_list_id: Uuid,
    market_id: Uuid,
}
