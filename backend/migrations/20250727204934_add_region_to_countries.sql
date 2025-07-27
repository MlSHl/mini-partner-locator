-- Add migration script here
alter table countries
add column region text check (region in ('APAC', 'EMEA', 'LATAM', 'NORAM'));

update countries set region='EMEA' where name != 'USA';
update countries set region='NORAM' where name = 'USA';

alter table countries
alter column region set not null;

