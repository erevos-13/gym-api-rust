use crate::schema::*;
use diesel::{ r2d2::ConnectionManager, sql_types::{ Text}, AsChangeset, Identifiable, Insertable, PgConnection, Queryable, QueryableByName, Selectable, Associations};
use serde::{Deserialize, Serialize};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(
    Debug,
    Insertable,
    QueryableByName,
    Queryable,
    Serialize,
    Selectable,
    Identifiable,
    Clone,
    PartialEq,
)]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(sql_type = Text)]
    pub id: String,
    pub username: String,
    pub age: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = password_users)]
pub struct PasswordUsers {
    pub id: String,
    pub user_id: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(
    Debug,
    Insertable,
    QueryableByName,
    Queryable,
    Serialize,
    Selectable,
    Identifiable,
    Clone,
    Deserialize,
)]
#[diesel(table_name = gym)]
pub struct Gym {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub address: String,
    pub postal_code: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(
    Debug,
    Insertable,
    QueryableByName,
    Queryable,
    Serialize,
    Selectable,
    Identifiable,
    Clone,
    Deserialize,
)]
#[diesel(table_name = activities)]
pub struct Activities {
    pub id: String,
    pub name: String,
    pub gym_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(
    Debug,
    Insertable,
    Queryable,
    Serialize,
    Selectable,
    Identifiable,
    Clone,
    Deserialize,
    AsChangeset,
    PartialEq
)]
#[diesel(table_name = slots)]
pub struct Slots {
    pub id: String,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub attendants: i32,
    pub gym_id: String,
    pub activity_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable, Clone, Deserialize, PartialEq,Associations)]
#[diesel(table_name = appointments)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Slots, foreign_key = slot_id))]
#[diesel(primary_key(user_id, gym_id, slot_id))]
pub struct Appointments {
    pub id: String,
    pub slot_id: String,
    pub gym_id: String,
    pub user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable, Clone, Deserialize, PartialEq,Associations)]
#[diesel(table_name = appointments_slots)]
#[diesel(belongs_to(Slots, foreign_key = slot_id))]
#[diesel(belongs_to(Appointments, foreign_key = appointment_id))]
#[diesel(primary_key(appointment_id, slot_id))]
pub struct AppointmentsSlots{
    pub slot_id: String,
    pub appointment_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}


#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable, Clone, Deserialize, PartialEq,Associations)]
#[diesel(table_name = users_gym)]
#[diesel(belongs_to(Gym, foreign_key = gym_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(user_id, gym_id))]
pub struct UsersGyms{
   pub user_id: String,
   pub gym_id: String,
   pub created_at: chrono::NaiveDateTime,
   pub updated_at: chrono::NaiveDateTime
}