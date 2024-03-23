#![allow(unused)]

use axum::{routing::get, Router};
use diesel::prelude::*;
use dotenv;

async fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let db_user = dotenv::var("DB_USER").unwrap();
    let db_pass = dotenv::var("DB_PASS").unwrap();
    let db_ip = dotenv::var("DB_IP").unwrap();
    let db_port = dotenv::var("DB_PORT").unwrap();
    let db_name = dotenv::var("DB_NAME").unwrap();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &db_user, &db_pass, &db_ip, &db_port, &db_name
    );
    println!("{}", db_url);

    PgConnection::establish(&db_url).unwrap_or_else(|e| panic!("Error connecting to {}", e))
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await;
    
}
