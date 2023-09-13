use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppointmentsInput {
    pub slot_id: String,
}
