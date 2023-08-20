use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::{
    r2d2::ConnectionManager, AsChangeset, FromSqlRow, Insertable, PgConnection, Queryable,
    QueryableByName, Selectable,
};
use serde::{Deserialize, Serialize};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    // pub age: i32,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
