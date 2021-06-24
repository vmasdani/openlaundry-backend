-- Your SQL goes here
create table backup_records (
    id integer primary key autoincrement,
    created_at timestamp default current_timestamp,
    updated_at timestamp,
    customers text,
    laundry_records text,
    laundry_documents text
);

create trigger if not exists ts_backup_records after insert on backup_records begin
    update backup_records set updated_at=current_timestamp where id=new.id;
end;