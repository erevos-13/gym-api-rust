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
    appointments (id) {
        id -> Varchar,
        slot_id -> Varchar,
        gym_id -> Varchar,
        user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    appointments_slots (appointment_id, slot_id) {
        slot_id -> Varchar,
        appointment_id -> Varchar,
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
    slots (id) {
        id -> Varchar,
        start_time -> Timestamp,
        end_time -> Timestamp,
        attendants -> Int4,
        gym_id -> Varchar,
        activity_id -> Varchar,
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

diesel::table! {
    users_gym (user_id, gym_id) {
        user_id -> Varchar,
        gym_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(appointments -> gym (gym_id));
diesel::joinable!(appointments -> slots (slot_id));
diesel::joinable!(appointments -> users (user_id));
diesel::joinable!(appointments_slots -> appointments (appointment_id));
diesel::joinable!(appointments_slots -> slots (slot_id));
diesel::joinable!(password_users -> users (user_id));
diesel::joinable!(slots -> activities (activity_id));
diesel::joinable!(slots -> gym (gym_id));
diesel::joinable!(users_gym -> gym (gym_id));
diesel::joinable!(users_gym -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    appointments,
    appointments_slots,
    gym,
    password_users,
    slots,
    testme,
    users,
    users_gym,
);
