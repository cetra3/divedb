
alter table photos add column width int not null default 0;
alter table photos add column height int not null default 0;
alter table photos add column description text not null default '';
alter table photos add column copyright text;