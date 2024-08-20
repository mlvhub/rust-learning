-- Add up migration script here
create table if not exists books (
    id serial primary key,
    name text not null
);

insert into books (name) values ('book1');
insert into books (name) values ('book2');
insert into books (name) values ('book3');
insert into books (name) values ('book4');
insert into books (name) values ('book5');
