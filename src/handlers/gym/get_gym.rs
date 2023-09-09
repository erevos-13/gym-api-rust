use crate::{
    input_model::gym_input_model::{GymRegister, GymsDTO},
    jwt_auth,
    models::{Gym, Pool},
};
use actix_web::{get, HttpMessage};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use diesel::{prelude::*, sql_types::Uuid};

#[get("/gym")]
pub async fn get_gym(
    req: HttpRequest,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap().clone();
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
            let gyms_dto = GymsDTO {
                gyms: gyms,
                total: total,
            };
            Ok(HttpResponse::Ok().json(gyms_dto))
        }
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(
    user_id: uuid::Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Gym>, crate::errors::ServiceError> {
    use crate::schema::gym::dsl::*;
    let gym_found = gym
        .select(user_id)
        .filter(user_id.eq(&user_id))
        .execute(conn)?;
    if gym_found >= 1 {
        let res: Vec<Gym> = gym.filter(user_id.eq(&user_id)).load::<Gym>(conn)?;
        return Ok(res);
    }
    return Err(crate::errors::ServiceError::BadRequest(
        "Gym does'n exists".to_string(),
    ));
}
