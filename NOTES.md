> cargo install diesel_cli --no-default-features --features  postgres

> diesel setup
> diesel migration generate create_items
> diesel migration run


```
create role jack LOGIN INHERIT;
grant postgres to jack;
alter user jack with password '123456';
create database jack;
alter user jack CREATEDB;

psql -d shop_base;

create table demotable (
    id Serial,
    name Varchar
)
;
```

```
diesel migration generate create_items;
```


```sql
create table items (
id serial primary key,
name varchar not null,
description varchar not null,
price integer not null,
instock interger not null default 0
)

```

```sql
drop table items;

```
