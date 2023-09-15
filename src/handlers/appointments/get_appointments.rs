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

fn query_appointments(id_user: String,conn: &mut PgConnection) -> Result<Vec<Appointments>, crate::errors::ServiceError> {
    use crate::schema::appointments::dsl::*;
    let gym_user_id = query_find_gym_user_id(id_user, conn)?;
    let res =  appointments.select(Appointments::as_select()).filter(gym_id.eq(gym_user_id.clone())).load::<Appointments>(conn)?;
    debug!("appointments found {:?}", res);
    Ok(res)
}

fn query_find_gym_user_id(user_selected_id: String, conn: &mut PgConnection) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::users_gym::dsl::*;
    let res = users_gym.select(gym_id).filter(user_id.eq(user_selected_id)).limit(1).get_result::<String>(conn)?;
    debug!("gym_user_id found {:?}", res);
    Ok(res)
}