use actix_web::{get, HttpRequest, HttpResponse, web};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::jwt_auth;
use crate::models::{Appointments, Pool};
use diesel::prelude::*;


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
            jwt.gym_id.to_string(),
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

fn query_appointments(gym_user_id:String,user_id: String,conn: &mut PgConnection) -> Result<Vec<Appointments>, crate::errors::ServiceError> {
    use crate::schema::appointments::dsl::*;
    let res =  appointments.select(Appointments::as_select()).filter(gym_id.eq(gym_user_id.clone())).load::<Appointments>(conn)?;
    debug!("appointments found {:?}", res);
    Ok(res)
}
