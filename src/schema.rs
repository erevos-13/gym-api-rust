// @generated automatically by Diesel CLI.

diesel::table! {
    testme (id) {
        id -> Int4,
        count -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        age -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    testme,
    users,
);
