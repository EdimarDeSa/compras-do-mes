-- Your SQL goes here
create table if not exists user_categorys (
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null unique,
    user_id uuid not null references users(id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now());

SELECT diesel_manage_updated_at('user_categorys');
