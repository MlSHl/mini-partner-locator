-- creating junction table between partners and countries
create table partner_countries (
    partner_id integer not null references partners(id) on delete cascade,
    country_id integer not null references countries(id) on delete cascade,
    primary key (partner_id, country_id)
);
