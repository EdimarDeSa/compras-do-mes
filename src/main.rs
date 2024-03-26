#![allow(unused)]

use axum::{routing::get, Router};
use chrono::prelude::*;
use diesel::prelude::*;
use std::collections::HashMap;
use std::process::exit;

use compras_do_mes::auth;
use compras_do_mes::connection;
use compras_do_mes::users::user_models::{AlterUser, NewUser};
use compras_do_mes::users::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
}
