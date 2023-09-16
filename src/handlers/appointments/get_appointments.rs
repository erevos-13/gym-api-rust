use actix_web::{get, HttpRequest, HttpResponse, web};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::jwt_auth;
use crate::models::{Appointments, Pool, Slots};
use diesel::prelude::*;
use crate::DTO::slots_appotiments_DTO::SlotsAppointmentsDTO;


#[get("/appointments")]
pub async fn get_appointments(
    req: HttpRequest,
    pool: web::Data<Pool>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query_appointments(
            jwt.user_id.to_string(),
            conn,
        )
    }).await?;
    debug!("result {:?}", result);
    match result {
        Ok(appointments) => Ok(HttpResponse::Ok().json(appointments)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query_appointments(id_user: String,conn: &mut PgConnection) -> Result<Vec<SlotsAppointmentsDTO>, crate::errors::ServiceError> {
    use crate::schema::appointments::dsl::*;

    let gym_user_id = query_find_gym_user_id(id_user, conn)?;
    let res =  appointments.select(Appointments::as_select()).filter(gym_id.eq(gym_user_id.clone())).load::<Appointments>(conn)?;
    let slots_ids = res.iter().map(|x| x.slot_id.clone()).collect::<Vec<String>>();
    let slots_found = query_found_slots(&slots_ids,conn)?;
    let app_slots_found = query_found_slots_by_appointment(slots_found, conn)?;
    debug!("appointments found {:?}", app_slots_found);
    Ok(app_slots_found)
}

fn query_find_gym_user_id(user_selected_id: String, conn: &mut PgConnection) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::users_gym::dsl::*;
    let res = users_gym.select(gym_id).filter(user_id.eq(user_selected_id)).limit(1).get_result::<String>(conn)?;
    Ok(res)
}

fn query_found_slots_by_appointment(slots: Vec<Slots>, conn: &mut PgConnection) -> Result<Vec<SlotsAppointmentsDTO>, crate::errors::ServiceError>{
    let appointments_found = Appointments::belonging_to(&slots).select(Appointments::as_select()).load(conn)?;
    let res = appointments_found.grouped_by(&slots).into_iter().zip(slots).map(|(appointment, slot)| SlotsAppointmentsDTO{ appointments: appointment, slots: slot }).collect::<Vec<SlotsAppointmentsDTO>>();
    debug!("query_found_slots_by_appointment found {:?}", res);
    Ok(res)
}

fn query_found_slots(slots_ids: &Vec<String>,conn: &mut PgConnection)-> Result<Vec<Slots>, crate::errors::ServiceError>{
    use crate::schema::slots::dsl::*;
    let res = slots.select(Slots::as_select()).filter(id.eq_any(slots_ids)).load(conn)?;
    Ok(res)
}

