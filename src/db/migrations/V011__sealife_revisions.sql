
create table if not exists sealife_revisions (
    sealife_id uuid REFERENCES sealife(id) ON DELETE SET NULL,
    name text not null,
    scientific_name text,
    description text not null,
    photo_id uuid REFERENCES photos(id) ON DELETE SET NULL,
    "date" timestamp with time zone not null,
    slug text not null
);
