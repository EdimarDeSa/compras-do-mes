-- Your SQL goes here
create table if not exists shopping_log (
    id uuid primary key default uuid_generate_v4(),
    date date not null default now(),
    user_id uuid not null references users(id),
    shopping_list_id uuid not null references shopping_list(id),
    market_id uuid not null references market(id),
    updated_at timestamp not null default now());

SELECT diesel_manage_updated_at('shopping_log');