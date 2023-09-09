// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Varchar,
        name -> Text,
        gym_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    gym (id) {
        id -> Varchar,
        name -> Text,
        address -> Text,
        postal_code -> Int4,
        user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    password_users (id) {
        id -> Varchar,
        user_id -> Varchar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
        role -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(gym -> users (user_id));
diesel::joinable!(password_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    gym,
    password_users,
    testme,
    users,
);
