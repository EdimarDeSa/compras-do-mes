-- Your SQL goes here
create table if not exists user_products (
    id uuid primary key default uuid_generate_v4(),
    name varchar(150) not null unique,
    unity_type_id uuid not null references unity_types(id),
    value money not null default '0.00',
    value_unity_type_id uuid not null references unity_types(id),
    category_id uuid not null references user_categorys(id),
    notes varchar(255),
    barcode varchar(50),
    image_url varchar(255),
    user_id uuid not null references users(id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now());

SELECT diesel_manage_updated_at('user_products');
