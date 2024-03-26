use axum::Router;

use compras_do_mes::users;
use compras_do_mes::auth;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .nest("/users", users::urls::create_routes())
        .nest("/auth", auth::urls::create_routes());

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
