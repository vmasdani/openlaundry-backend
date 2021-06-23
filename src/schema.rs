table! {
    users (uid) {
        uid -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}
