-- Remove city and country from partners
alter table partners
drop column country;

alter table partners
drop column city;
