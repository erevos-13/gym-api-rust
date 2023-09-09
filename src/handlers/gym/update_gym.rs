use crate::{
    input_model::gym_input_model::GymRegister,
    jwt_auth,
    models::{Gym, Pool},
};
use actix_web::{patch, web, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;

#[patch("/gym/{id}")]
pub async fn update_gym(
    id: web::Path<String>,
    gym_update: web::Json<GymRegister>,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    info!("Update gym: {:?}, with id: {:?}", gym_update, id);
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(id.to_string().clone(), &gym_update, conn)
    })
    .await?;

    match result {
        Ok(gym_updated) => Ok(HttpResponse::Ok().json(gym_updated)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(
    _id: String,
    gym_update: &GymRegister,
    conn: &mut PgConnection,
) -> Result<Gym, crate::errors::ServiceError> {
    use crate::schema::gym::dsl::*;
    let gym_found = gym.select(id).filter(id.eq(&_id)).execute(conn)?;
    if gym_found >= 1 {
        let res: Gym = diesel::update(gym.filter(id.eq(&_id)))
            .set((
                name.eq(&gym_update.name),
                address.eq(&gym_update.address),
                postal_code.eq(&gym_update.postal_code),
                updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<Gym>(conn)?;
        return Ok(res);
    }
    return Err(crate::errors::ServiceError::BadRequest(
        "Gym does'n exists".to_string(),
    ));
}
