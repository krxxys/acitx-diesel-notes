// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        user_id -> Int4,
        note_name -> Varchar,
        note_tag -> Varchar,
        note_data -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(notes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
