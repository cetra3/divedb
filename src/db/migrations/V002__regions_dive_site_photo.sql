

alter table dive_sites add column photo_id uuid REFERENCES photos(id) ON DELETE SET NULL;
alter table photos add column size int not null;
alter table photos alter column "date" set default now();
alter table photos alter column "date" set not null;

create table if not exists regions (
    id uuid primary key,
    name text not null,

    lat_min double precision not null,
    lon_min double precision not null,
    lat_max double precision not null,
    lon_max double precision not null
)