use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, Queryable, Debug, Validate)]
pub struct GymRegister {
    pub name: String,
    pub address: String,
    pub postal_code: i32,
}
#[derive(Deserialize, Clone, Queryable, Debug, Validate, Serialize)]
pub struct ResponseDTO<T> {
    pub data: Vec<T>,
    pub total: usize,
}
