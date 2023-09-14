use actix_web::{get, HttpRequest, HttpResponse, web};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::jwt_auth;
use crate::models::{Appointments, Pool, Slots, User};
use crate::schema::appointments::dsl::appointments;
use diesel::prelude::*;
use crate::schema::slots::dsl::slots;

#[get("/appointments/my")]
pub async fn get_appointments_by_user( req: HttpRequest,
                                       pool: web::Data<Pool>,
                                       jwt: jwt_auth::JwtMiddleware,) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query_appointments(
            jwt.gym_id.to_string(),
            jwt.user_id.to_string(),
            conn,
        )
    }).await?;
    debug!("result {:?}", result);
    match result {
        Ok(appointments_found) => Ok(HttpResponse::Ok().json(appointments_found)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string()))
    }
}

fn query_appointments(app_gym_id: String,app_user_id: String,conn: &mut PgConnection) -> Result<Vec<Appointments>,crate::errors::ServiceError> {
    use crate::schema::appointments::dsl::*;
    let get_user = get_user(app_user_id.clone(), conn)?;
    let res = Appointments::belonging_to(&get_user).load::<Appointments>(conn)?;
    // let res = appointments.select(Appointments::as_select()).filter(gym_id.eq(app_gym_id)).filter(user_id.eq(app_user_id)).get_results(conn)?;
    debug!("appointments found {:?}, slots:", res);


    Ok(res)
}

fn get_user(app_user_id: String, conn: &mut PgConnection) -> Result<User, crate::errors::ServiceError> {
    use crate::schema::users::dsl::*;
    let get_user = users.select(User::as_select()).filter(id.eq(app_user_id.clone())).get_result::<User>(conn)?;
    Ok(get_user)
}