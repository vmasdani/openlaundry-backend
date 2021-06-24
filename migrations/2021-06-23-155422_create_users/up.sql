-- Your SQL goes here
create table users (
    uid text primary key,
    created_at timestamp default current_timestamp,
    updated_at timestamp,
    name text,
    email text
);

create trigger if not exists users_ts after update on users begin
    update users set updated_at=current_timestamp where id=new.id;
end;

