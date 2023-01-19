
alter table sealife_category_values drop column category_value_id;
alter table sealife_category_values add column category_value_id uuid not null REFERENCES category_values(id) ON DELETE CASCADE