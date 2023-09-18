// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        #[max_length = 200]
        title -> Varchar,
        content -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 200]
        password -> Varchar,
        #[max_length = 50]
        username -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
