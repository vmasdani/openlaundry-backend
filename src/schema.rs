table! {
    backup_records (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        customers -> Nullable<Text>,
        laundry_records -> Nullable<Text>,
        laundry_documents -> Nullable<Text>,
        email -> Nullable<Text>,
        expenses -> Nullable<Text>,
    }
}

table! {
    users (uid) {
        uid -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    backup_records,
    users,
);
