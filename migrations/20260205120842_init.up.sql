-- Add up migration script here
create table if not exists urls (
    id serial primary key,
    short_url text not null unique,
    long_url text not null,
    tag varchar(255) not null,
    ttl bigint not null,
    created_at timestamp not null default now()
);
