alter table regions add column slug text;

update regions set slug = slugify(name);

alter table regions alter column slug set not null;