create table if not exists email_verification (
    id uuid primary key,
    user_id uuid not null REFERENCES users(id) ON DELETE CASCADE,
    "date" timestamp with time zone not null default now()
);
