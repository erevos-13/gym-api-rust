use diesel::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, Queryable, Debug, Validate, Serialize)]

pub struct SlotsInput {
    pub start_time: i64,
    pub end_time: i64,
    pub activity_id: String,
    pub attendants: i32,
}
