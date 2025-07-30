-- Add migration script here
alter table partners
alter column website_url set not null;
