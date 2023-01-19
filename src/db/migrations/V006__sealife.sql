
create table if not exists sealife (
    id uuid primary key,
    name text not null,
    scientific_name text,
    description text not null,
    photo_id uuid REFERENCES photos(id) ON DELETE SET NULL,
    "date" timestamp with time zone not null,
    slug text not null
);

create table if not exists sealife_tags (
    sealife_id uuid not null REFERENCES sealife(id) ON DELETE CASCADE,
    photo_id uuid not null REFERENCES photos(id) ON DELETE CASCADE,

    u_min double precision not null,
    v_min double precision not null,
    u_max double precision not null,
    v_max double precision not null
);

create table if not exists categories (
    id uuid primary key,
    name text not null
);

create table if not exists category_values (
    id uuid primary key,
    category_id uuid not null REFERENCES categories(id) ON DELETE CASCADE,
    value text not null
);

create table if not exists sealife_category_values (
    id uuid primary key,
    sealife_id uuid not null REFERENCES sealife(id) ON DELETE CASCADE,
    category_value_id uuid not null REFERENCES categories(id) ON DELETE CASCADE
);
