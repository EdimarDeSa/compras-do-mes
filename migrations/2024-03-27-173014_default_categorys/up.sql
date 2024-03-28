-- Your SQL goes here
create table if not exists default_categorys (
    id uuid primary key default uuid_generate_v4(),
    name varchar(100) not null unique
);

INSERT INTO default_categorys (name) VALUES
    ('Frios'),
    ('Laticínios'),
    ('Carnes'),
    ('Hortifruti'),
    ('Bebidas'),
    ('Limpeza'),
    ('Higiene Pessoal'),
    ('Padaria'),
    ('Cereais e Grãos'),
    ('Enlatados');

