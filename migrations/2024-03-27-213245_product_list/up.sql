-- Your SQL goes here
create table if not exists product_list (
    id uuid primary key default uuid_generate_v4(),
    shopping_list_id uuid not null references shopping_list(id),
    user_product_id uuid not null references user_products(id),
    quantity real not null default '0',
    value money not null,
    total money not null,
    on_cart boolean default false,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now());

SELECT diesel_manage_updated_at('product_list');
