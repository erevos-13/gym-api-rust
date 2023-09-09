use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::Gym;

#[derive(Deserialize, Clone, Queryable, Debug, Validate)]
pub struct GymRegister {
    pub name: String,
    pub address: String,
    pub postal_code: i32,
}
#[derive(Deserialize, Clone, Queryable, Debug, Validate, Serialize)]
pub struct GymsDTO {
    pub gyms: Vec<Gym>,
    pub total: usize,
}
