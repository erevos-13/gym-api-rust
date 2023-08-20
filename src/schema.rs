// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
