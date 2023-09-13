use crate::{
    input_model::gym_input_model::{GymRegister, ResponseDTO},
    jwt_auth,
    models::{Gym, Pool},
};
use actix_web::{get, HttpMessage};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;

#[get("/gym")]
pub async fn get_gym(
    req: HttpRequest,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap().clone();
    info!("Get gym with user_id: {:?}", &user_id);
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(user_id, conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(gyms) => {
            let total = gyms.len();
            let gyms_dto = ResponseDTO {
                data: gyms,
                total: total,
            };
            Ok(HttpResponse::Ok().json(gyms_dto))
        }
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(
    _id: uuid::Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Gym>, crate::errors::ServiceError> {
    use crate::schema::gym::dsl::*;

    let gyms_found = gym
        .select(gym::all_columns())
        .filter(user_id.eq(&_id.to_string()))
        .load::<Gym>(conn);
    info!("Gyms found: {:?}", gyms_found);
    match gyms_found {
        Ok(gyms) => Ok(gyms),
        Err(e) => Err(crate::errors::ServiceError::BadRequest(e.to_string())),
    }
}
