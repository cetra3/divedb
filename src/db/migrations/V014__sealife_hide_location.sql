alter table sealife add column hide_location boolean not null default false;

-- This removes duplicates
with u as (select distinct * from sealife_tags), x as (delete from sealife_tags) insert into sealife_tags select * from u;
alter table sealife_tags add primary key (sealife_id, photo_id);