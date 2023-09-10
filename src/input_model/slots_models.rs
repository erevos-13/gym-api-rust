use diesel::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, Queryable, Debug, Validate, Serialize)]

pub struct SlotsInput {
    pub start_time: u32,
    pub end_time: u32,
    pub activity_id: String,
    pub attendants: i32,
}
