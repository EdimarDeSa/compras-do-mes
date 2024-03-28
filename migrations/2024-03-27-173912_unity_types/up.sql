-- Your SQL goes here
create table if not exists unity_types (
    id uuid primary key default uuid_generate_v4(),
    name varchar(25) not null unique,
    nick varchar(10) not null,
    calc_value smallint not null
);

INSERT INTO unity_types (name, nick, calc_value) VALUES
    ('Gramas', 'g', 1),
    ('Kilos', 'kg', 1000),
    ('Pacote', 'pac', 1),
    ('Unidade', 'und', 1),
    ('Litros', 'l', 1000),
    ('Mililitros', 'ml', 1),
    ('Dúzia', 'dz', 12),
    ('Caixa', 'cx', 1),
    ('Bandeja', 'bdj', 1),
    ('Galão', 'gal', 3785);
