alter table photos add column upload_date timestamp with time zone not null default now();