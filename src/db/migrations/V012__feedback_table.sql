create table if not exists feedback (
    id uuid primary key,
    "date" timestamp with time zone not null default now(),
    user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    feedback text not null
);