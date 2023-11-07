
alter table users drop constraint users_username_key;

alter table users alter column email drop not null;

alter table users add column external boolean not null default false;
alter table users add column ap_id text;
alter table users add column inbox text;


create table if not exists followers(
  user_id uuid not null REFERENCES users(id) ON DELETE CASCADE,
  followed_by_id uuid not null REFERENCES users(id) ON DELETE CASCADE,
  "date" timestamp with time zone not null default now()
);

alter table followers add primary key (user_id, followed_by_id);