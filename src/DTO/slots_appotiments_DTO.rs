use serde::{Deserialize, Serialize};
use crate::models::{Appointments, Slots};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlotsAppointmentsDTO {

   pub appointments: Vec<Appointments>,
    #[serde(flatten)]
   pub slots: Slots,
}