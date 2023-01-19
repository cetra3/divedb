
alter table photos add column dive_site_id uuid REFERENCES dive_sites(id) ON DELETE SET NULL;

update photos set dive_site_id = dives.dive_site_id from dives where dives.id = photos.dive_id;