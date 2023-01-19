
alter table dive_sites add column "date" timestamp with time zone not null default now();