#![allow(unused)]

use axum::{routing::get, Router};
use chrono::prelude::*;
use diesel::prelude::*;
use std::process::exit;

use compras_do_mes::auth;
use compras_do_mes::connection;
use compras_do_mes::users::user_models::{AlterUser, NewUser};
use compras_do_mes::users::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let email = "email_super.criativo@queemail.com".to_string();
    let new_user = NewUser {
        nickname: "Fulanisson de Teste".to_string(),
        email: email.clone(),
        password: "Senh@Mu1toF0rt3".to_string(),
        birth_date: Some(NaiveDate::parse_from_str("1995-09-04", "%Y-%m-%d").unwrap()),
    };

    match create_user::new(&new_user) {
        Ok(u) => println!("Usuário criado com sucesso! \n {:?}", u),
        Err(e) => println!("Erro ao criar usuário: {:?}", e),
    };

    let user = read_user::find_with_email(&email).unwrap();
    println!("Usuário encontrado: {:?}", user);

    let token = auth::login(&new_user.email, &new_user.password).unwrap();
    println!("Token gerado: {:?}", token);

    let verifying = auth::check_jwt_token(&token.token, &user.id.to_string());
    println!("Token verificado: {:?}", verifying);

    let alter_user = AlterUser {
        id: user.id,
        nickname: Some("Fulanisson de Teste Alterado".to_string()),
        email: None,
        password: None,
        birth_date: None,
    };
    let altered = update_user::update(alter_user).unwrap();
    println!("Usuário alterado: {:?}", altered);

    let user = read_user::find_with_email(&email).unwrap();
    println!("Usuário encontrado: {:?}", user);

    let deletion = delete_user::remove(&user.id);
    println!("{:?}", deletion);
}
