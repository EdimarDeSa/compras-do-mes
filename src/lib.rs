pub mod schema;

pub mod connection;

pub mod auth;

pub mod users {
    pub mod create_user;
    pub mod delete_user;
    pub mod read_user;
    pub mod user_models;
}

pub mod constants {
    pub const ID: &str = "id";

    pub const DB_USER: &str = "DB_USER";
    pub const DB_PASS: &str = "DB_PASS";
    pub const DB_IP: &str = "DB_IP";
    pub const DB_PORT: &str = "DB_PORT";
    pub const DB_NAME: &str = "DB_NAME";

    pub const JWT_SECRET: &str = "JWT_SECRET";
}
