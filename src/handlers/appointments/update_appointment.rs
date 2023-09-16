use actix_web::{HttpRequest, HttpResponse, patch, web};
use actix_web::web::Json;
use diesel::associations::HasTable;
use diesel::PgConnection;
use crate::input_model::appointment_model::{AppointmentsUpdateInput};
use crate::jwt_auth;
use crate::models::{Appointments, Pool, Slots};
use diesel::prelude::*;
use crate::DTO::slots_appotiments_DTO::SlotsAppointmentsDTO;

#[patch("/appointments")]
pub async fn update_appointment(
    req: HttpRequest,
    appointment_input:Json<AppointmentsUpdateInput>,
    pool: web::Data<Pool>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("update_appointment: {:?}", appointment_input);
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query_appointments(
            jwt.user_id.to_string(),
            appointment_input,
            conn,
        )
    }).await.map_err(|e| {
        warn!("error on update appointment{}", e);
        HttpResponse::InternalServerError().finish()
    });
      match result {
        Ok(appointment) => Ok(HttpResponse::Ok().json(appointment)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e)),
    }
}

fn query_appointments(id_user: String, update_appointment_input: Json<AppointmentsUpdateInput>, conn: &mut PgConnection) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::{appointments, slots};

    let appointment_with_slot = appointments::table
        .inner_join(slots::table)
        .filter(slots::id.eq(update_appointment_input.slot_id.clone()))
        .limit(1)
        .select((Appointments::as_select(), Slots::as_select()))
        .load::<(Appointments, Slots)>(conn)?;
    if appointment_with_slot.len() == 0 {
        return Err(crate::errors::ServiceError::BadRequest(
            "Error updating attendants".into(),
        ));
    }
    let update_slot = diesel::update(slots::table)
        .filter(slots::id.eq(update_appointment_input.slot_id.clone()))
        .set(slots::attendants.eq(slots::attendants + 1))
        .execute(conn);
    match update_slot {
        Ok(_) => {
            return query_delete_appointment(update_appointment_input,conn);
        },
        Err(_) => return Err(crate::errors::ServiceError::BadRequest(
            "Error updating attendants".into(),
        )),
    }
}

fn query_delete_appointment(update_appointment_input: Json<AppointmentsUpdateInput>, conn: &mut PgConnection) ->Result<String, crate::errors::ServiceError> {
    use crate::schema::appointments::dsl::*;
    let res =  diesel::delete(appointments::table)
        .filter(appointments::id.eq(update_appointment_input.appointment_id.clone()))
        .execute(conn);
    match res {
        Ok(_) => Ok(format!("Appointment {:?} deleted",update_appointment_input.appointment_id)),
        Err(_) => Err(crate::errors::ServiceError::BadRequest(
            "Error updating attendants".into(),
        ))
    }
}