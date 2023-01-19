
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'Difficulty') THEN
        CREATE TYPE "Difficulty" as enum ('OW', 'AOW', 'Tech');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'UserLevel') THEN
        CREATE TYPE "UserLevel" as enum ('User', 'Editor', 'Admin');
    END IF;
END$$;

create table if not exists users (
    id uuid primary key,
    email text unique not null,
    password text not null,
    level "UserLevel" not null
);

create table if not exists dive_sites (
    id uuid primary key,
    user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    name text not null,
    description text not null,
    access text not null,
    difficulty "Difficulty" not null,
    depth double precision not null,
    lat double precision not null,
    lon double precision not null,
    published boolean not null
);

create table if not exists dive_sites_revision (
    dive_id uuid REFERENCES dive_sites(id) ON DELETE CASCADE,
    user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    "date" timestamp with time zone not null default now(),
    name text not null,
    description text not null,
    access text not null,
    difficulty "Difficulty" not null,
    depth double precision not null,
    lat double precision not null,
    lon double precision not null,
    published boolean not null
);


create table if not exists dives (
    id uuid primary key,
    user_id uuid REFERENCES users(id) ON DELETE CASCADE,
    "date" timestamp with time zone,
    duration integer,
    depth real,
    dive_site_id uuid REFERENCES dive_sites(id) ON DELETE SET NULL
);

create table if not exists dive_metrics (
    dive_id uuid REFERENCES dives(id) ON DELETE CASCADE,
    "time" integer not null,
    depth real not null,
    pressure real,
    temperature real
);

create table if not exists photos (
    id uuid primary key,
    user_id uuid REFERENCES users(id) ON DELETE CASCADE,
    filename text not null,
    "date" timestamp with time zone,
    dive_id uuid REFERENCES dives(id) ON DELETE SET NULL
);

create index if not exists dive_metrics_dive_id on dive_metrics (dive_id);