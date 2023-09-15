use crate::{
    input_model::gym_input_model::GymRegister,
    jwt_auth,
    models::{Activities, Gym, Pool},
};
use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;

#[get("/activities")]
pub async fn get_activities(
    req: HttpRequest,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let user_clone = user_id.clone();
    info!("Get activities with user_id: {:?}", &user_id);

    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(user_clone, conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(activities) => Ok(HttpResponse::Ok().json(activities)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(
    _id: uuid::Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<Activities>, crate::errors::ServiceError> {
    use crate::schema::activities::dsl::*;
    let gym_id_by_user = query_get_gym_by_user(_id.to_string(), conn)?;
    let activities_found = activities
        .select(activities::all_columns())
        .filter(gym_id.eq(&gym_id_by_user))
        .get_results(conn)?;
    info!("Activities found: {:?}", activities_found);
    Ok(activities_found)
}

fn query_get_gym_by_user(
    user_has_id: String,
    conn: &mut PgConnection,
) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::gym::dsl::*;
    // TODO change to query_get_gym_by_user and add the user_gym
    let gym_found = gym
        .select(gym::all_columns())
        .load::<Gym>(conn)?;
    debug!("gym_found: {:?}", gym_found);
    if gym_found.len() == 0 {
        return Err(crate::errors::ServiceError::BadRequest(
            "Gym does'n exists".to_string(),
        ));
    }
    Ok(gym_found[0].id.clone())
}
