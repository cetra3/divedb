alter table dives add column dive_number integer not null default 0;
alter table dives add column description text not null default '';
alter table dives add column published boolean not null default false;

--- Add a dive number to each dive
update dives set dive_number = rn
  from (
    select id, row_number() over (partition by user_id order by date) as rn
    from dives
  ) as subquery
where dives.id = subquery.id;

create table if not exists dive_likes (
  dive_id uuid not null REFERENCES dives(id) ON DELETE CASCADE,
  user_id uuid not null REFERENCES users(id) ON DELETE CASCADE,
  "date" timestamp with time zone not null default now()
);

alter table dive_likes add primary key (dive_id, user_id);

create index if not exists dive_likes_dive_id on dive_likes (dive_id);
create index if not exists dive_likes_user_id on dive_likes (user_id);

create table if not exists dive_comments (
  id uuid primary key,
  dive_id uuid not null REFERENCES dives(id) ON DELETE CASCADE,
  user_id uuid not null REFERENCES users(id) ON DELETE CASCADE,
  description text not null,
  "date" timestamp with time zone not null default now()
);

create index if not exists dive_comments_dive_id on dive_comments (dive_id);
create index if not exists dive_comments_user_id on dive_comments (user_id);

create table if not exists photo_likes (
  photo_id uuid not null REFERENCES photos(id) ON DELETE CASCADE,
  user_id uuid not null REFERENCES users(id) ON DELETE CASCADE,
  "date" timestamp with time zone not null default now()
);

alter table photo_likes add primary key (photo_id, user_id);

create index if not exists photo_likes_photo_id on photo_likes (photo_id);
create index if not exists photo_likes_user_id on photo_likes (user_id);

--- Adjusts the schema to make sure that there is now a unique `username` and moves the existing field to `display_name`
alter table users add column display_name text;

update users set display_name = username;
update users set username = lower(regexp_replace(email, '@.*$', '')) where username is null;
update users set username = slugify(username) where username is not null;

--- This filters out any duplicates
update users
set username = users.username || '_' || rn 
  from (
    select id, username, row_number() over (partition by username order by username) as rn
    from users
  ) as subquery
where users.id = subquery.id
and rn > 1;

alter table users alter column username set not null;
alter table users add unique(username);
alter table users alter column password drop not null;