use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppointmentsInput {
    pub slot_id: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub enum AppointmentStatus {
    Pending = 1,
    Confirmed = 2,
    Cancelled = 3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppointmentsUpdateInput {
    pub slot_id: String,
    pub appointment_id: String,
    pub status: AppointmentStatus,
}