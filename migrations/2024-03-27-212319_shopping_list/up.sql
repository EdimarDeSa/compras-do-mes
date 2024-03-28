-- Your SQL goes here
create table if not exists shopping_list (
    id uuid primary key default uuid_generate_v4(),
    name varchar(150) not null unique,
    user_id uuid not null references users(id),
    final_value money not null,
    unique_itens smallint not null,
    total_itens smallint not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now());

SELECT diesel_manage_updated_at('shopping_list');
