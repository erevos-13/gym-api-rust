use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::FromSql,
    pg::Pg,
    r2d2::ConnectionManager,
    sql_types::{BigInt, Text},
    AsChangeset, FromSqlRow, Identifiable, Insertable, PgConnection, Queryable, QueryableByName,
    Selectable,
};
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
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = gym)]
pub struct Gym {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub address: String,
    pub postal_code: i32,
    #[diesel(sql_type = Text)]
    pub user_id: String,
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

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable, Clone, Deserialize)]
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
