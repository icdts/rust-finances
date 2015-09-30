drop table if exists accounts;
create table accounts (
  id integer primary key autoincrement,
  name text not null
);

drop table if exists categories;
create table categories (
  id integer primary key autoincrement,
  name text not null
);

drop table if exists transactions;
create table transactions (
  id integer primary key autoincrement,
  account_id integer,
  created_date date,
  description text,
  amount integer
);

drop table if exists cateogry_transaction;
create table category_transaction (
    id integer primary key autoincrement,
    category_id integer,
    transaction_id integer
)

