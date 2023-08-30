use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::FromSql, pg::Pg, r2d2::ConnectionManager, sql_types::BigInt, AsChangeset,
    FromSqlRow, Identifiable, Insertable, PgConnection, Queryable, QueryableByName, Selectable,
};
use serde::{Deserialize, Serialize};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable, Clone)]
#[diesel(table_name = users)]
pub struct User {
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
