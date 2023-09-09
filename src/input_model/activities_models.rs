use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, Queryable, Debug, Validate, Serialize)]
pub struct ActivityInput {
    pub name: String,
    pub gym_id: String,
}
