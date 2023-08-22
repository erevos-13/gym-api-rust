use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::FromSql, pg::Pg, r2d2::ConnectionManager, sql_types::BigInt, AsChangeset,
    FromSqlRow, Identifiable, Insertable, PgConnection, Queryable, QueryableByName, Selectable,
};
use serde::{Deserialize, Serialize};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub age: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable, Serialize, Selectable, Identifiable)]
#[diesel(table_name = testme)]
pub struct Testme {
    pub id: i32,
    pub count: i32,
}
