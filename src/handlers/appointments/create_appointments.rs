use actix_web::{
    post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use chrono::Utc;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
    input_model::appointment_model::AppointmentsInput,
    jwt_auth,
    models::{Appointments, Pool, Slots},
};
use diesel::SelectableHelper;

#[post("/appointments")]
pub async fn create_appointments(
    req: HttpRequest,
    appointment_input: web::Json<AppointmentsInput>,
    pool: web::Data<Pool>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query_appointments(
            jwt.gym_id.to_string(),
            jwt.user_id.to_string(),
            appointment_input,
            conn,
        )
    })
    .await?;

    match result {
        Ok(appointment) => Ok(HttpResponse::Ok().json(appointment)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}
fn query_find_and_remove_attendant_slot(
    slot_id: String,
    conn: &mut PgConnection,
) -> Result<bool, crate::errors::ServiceError> {
    use crate::schema::slots::dsl::*;

    let attendants_found = slots
        .filter(id.eq(slot_id.clone()))
        .limit(1)
        .select(Slots::as_select())
        .load(conn)
        .expect("Error loading posts");
    let attendants_count = attendants_found[0].attendants;

    debug!(
        "attendants found count {:?} for slot id {:?}",
        attendants_count, slot_id
    );
    let res = diesel::update(slots.filter(id.eq(slot_id)))
        .set(attendants.eq(attendants_count - 1))
        .execute(conn);
    match res {
        Ok(_) => Ok(true),
        Err(_) => Err(crate::errors::ServiceError::BadRequest(
            "Error updating attendants".into(),
        )),
    }
}

fn query_can_book_this_appointment(
    slot_selected_id: String,
    conn: &mut PgConnection,
) -> Result<bool, crate::errors::ServiceError> {
    use crate::schema::slots::dsl::*;
    let attendants_position = slots
        .filter(id.eq(slot_selected_id))
        .limit(1)
        .select(Slots::as_select())
        .load(conn);
    match attendants_position {
        Ok(slot_found) => {
            let attendants_count = slot_found[0].attendants;
            if attendants_count == 0 {
                return Err(crate::errors::ServiceError::BadRequest(
                    "No more attendants for this slot".into(),
                ));
            }
            Ok(true)
        }
        Err(_) => {
            return Err(crate::errors::ServiceError::BadRequest(
                "Error finding attendants".into(),
            ));
        }
    }
}
fn query_appointments(
    jwt_gym_id: String,
    jwt_user_id: String,
    appoint: Json<AppointmentsInput>,
    conn: &mut PgConnection,
) -> Result<Appointments, crate::errors::ServiceError> {
    use crate::schema::appointments::dsl::*;

    let user_selected_this_appointment = appointments
        .filter(slot_id.eq(appoint.slot_id.clone()))
        .filter(user_id.eq(jwt_user_id.clone()))
        .limit(1)
        .select(Appointments::as_select())
        .load(conn)?;
    if user_selected_this_appointment.len() > 0 {
        return Err(crate::errors::ServiceError::BadRequest(
            "User already selected this appointment".into(),
        ));
    }
    query_can_book_this_appointment(appoint.slot_id.clone(), conn)?;

    let new_appointment = Appointments {
        id: uuid::Uuid::new_v4().to_string(),
        slot_id: appoint.slot_id.clone(),
        gym_id: jwt_gym_id,
        user_id: jwt_user_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let res = diesel::insert_into(appointments)
        .values(&new_appointment)
        .get_result(conn)?;
    query_find_and_remove_attendant_slot(appoint.slot_id.clone(), conn)?;
    Ok(res)
}
