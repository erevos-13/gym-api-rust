use crate::{
    input_model::{activities_models::ActivityInput, gym_input_model::GymRegister},
    jwt_auth,
    models::{Activities, Gym, Pool},
    schema::{
        activities::{self, *},
        gym::*,
    },
};
use actix_web::{delete, patch, HttpMessage};
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::{associations::HasTable, delete, insert_into, prelude::*, update};

#[delete("/activities/{activity_id}")]
pub async fn delete_activity(
    activity_id: web::Path<String>,
    req: HttpRequest,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(activity_id.to_string(), conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(activity) => Ok(HttpResponse::Ok().json(activity)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(
    activity_id: String,
    conn: &mut PgConnection,
) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::activities::dsl::*;
    let activity_found = activities
        .select(id)
        .filter(id.eq(&activity_id))
        .execute(conn)?;
    if activity_found >= 1 {
        delete(activities.filter(id.eq(&activity_id))).get_result::<Activities>(conn)?;
        return Ok("Activity deleted".to_string());
    }
    return Err(crate::errors::ServiceError::BadRequest(
        "Activity does'n exists".to_string(),
    ));
}
