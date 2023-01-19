create table if not exists og_reference (
    id uuid primary key,
    url text not null,
    title text not null,
    image_url text,
    description text not null,
    last_fetched timestamp with time zone not null default now()
);

create table if not exists sealife_og_reference (
    sealife_id uuid not null REFERENCES sealife(id) ON DELETE CASCADE,
    og_reference_id uuid not null REFERENCES og_reference(id) ON DELETE CASCADE
);

create table if not exists dive_sites_og_reference (
    dive_site_id uuid not null REFERENCES dive_sites(id) ON DELETE CASCADE,
    og_reference_id uuid not null REFERENCES og_reference(id) ON DELETE CASCADE
);

create index if not exists sealife_og_reference_id on sealife_og_reference (sealife_id);
create index if not exists dive_sites_og_reference_id on dive_sites_og_reference (dive_site_id);