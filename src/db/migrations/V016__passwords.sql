create table if not exists password_reset (
    id uuid primary key,
    user_id uuid not null REFERENCES users(id) ON DELETE CASCADE,
    "date" timestamp with time zone not null default now()
);


alter table users add column email_verified boolean not null default false;