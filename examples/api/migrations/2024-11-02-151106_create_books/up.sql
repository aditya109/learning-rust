-- Your SQL goes here
create table books (
    id serial primary key ,
    title varchar not null,
    author varchar not null,
    published boolean not null default 'f'
)