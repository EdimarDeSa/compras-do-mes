-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id uuid primary key default uuid_generate_v4(),
    nickname varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(60) not null,
    birth_date date,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
    );

SELECT diesel_manage_updated_at('users');

insert into users (nickname, email, password, birth_date) values
('example', 'example@example.com', '$2b$12$F6D65Ct5SHunnOlIuUy84ea6dZHbzg5kARDXOVHjTMZFaUzqL5Qku', '1990-01-01')