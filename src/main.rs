#![allow(unused)]

use axum::{routing::get, Router};
use chrono::prelude::*;
use diesel::prelude::*;

use compras_do_mes::connection;
use compras_do_mes::models::NewUser;
use compras_do_mes::users::auth;
use compras_do_mes::users::create_user;
use compras_do_mes::users::read_user;

#[tokio::main]
async fn main() {
    let conn = &mut connection::establish_connection();
    let email = "edimarfreitas95@gmail.com".to_string();

    let new_user = NewUser {
        nickname: "Edimar".to_string(),
        email: email.clone(),
        password: "048Edim@r258".to_string(),
        birth_date: Some(NaiveDate::parse_from_str("1995-09-04", "%Y-%m-%d").unwrap()),
    };

    match create_user::new(conn, &new_user) {
        Ok(u) => println!("Usuário criado com sucesso! \n {:?}", u),
        Err(e) => println!("Erro ao criar usuário: {:?}", e),
    };

    let user = read_user::find(conn, &email).unwrap();
    println!("Usuário encontrado: {:?}", user);

    let token = auth::login(conn, &new_user.email, &new_user.password).unwrap();
    println!("Token gerado: {:?}", token);

    let verifying = auth::check_jwt_token(&token.token, &user.id.to_string());
    println!("Token verificado: {:?}", verifying);
}
