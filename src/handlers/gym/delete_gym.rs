use crate::{

    models::{Gym, Pool},
};
use actix_web::{delete};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

#[delete("/gym/{id}")]
pub async fn delete_gym(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("Delete gym with id: {:?}", id);
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(id.to_string().clone(), conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(gym_deleted) => Ok(HttpResponse::Ok().json(gym_deleted)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(_id: String, conn: &mut PgConnection) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::gym::dsl::*;
    let gym_found = gym.select(id).filter(id.eq(&_id)).execute(conn)?;
    if gym_found >= 1 {
        diesel::delete(gym.filter(id.eq(&_id))).get_result::<Gym>(conn)?;
        return Ok("Gym deleted".to_string());
    }
    return Err(crate::errors::ServiceError::BadRequest(
        "Gym does'n exists".to_string(),
    ));
}
