use crate::{
    input_model::{activities_models::ActivityInput, gym_input_model::GymRegister},
    jwt_auth,
    models::{Activities, Gym, Pool},
    schema::{
        activities::{self, *},
        gym::*,
    },
};
use actix_web::{patch, HttpMessage};
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::{associations::HasTable, insert_into, prelude::*, update};

#[patch("/activities/{activity_id}")]
pub async fn update_activity(
    activity_id: web::Path<String>,
    req: HttpRequest,
    activity: web::Json<ActivityInput>,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(activity_id.to_string(), &activity, conn)
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
    activity: &ActivityInput,
    conn: &mut PgConnection,
) -> Result<Activities, crate::errors::ServiceError> {
    use crate::schema::activities::dsl::*;
    let same_name_activity_exist = activities
        .select(id)
        .filter(id.eq(&activity_id))
        .filter(gym_id.eq(&activity.gym_id))
        .execute(conn)?;
    if same_name_activity_exist == 0 {
        return Err(crate::errors::ServiceError::BadRequest(
            "Activity is not exist".to_string(),
        ));
    }

    let activitt_save = diesel::update(activities.filter(id.eq(&activity_id)))
        .set((
            name.eq(&activity.name),
            gym_id.eq(&activity.gym_id),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .get_result::<Activities>(conn)?;
    Ok(activitt_save)
}
