> cargo install diesel_cli --no-default-features --features  postgres

> diesel setup
> diesel migration generate create_items

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
