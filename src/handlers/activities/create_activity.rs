use crate::{
    input_model::{activities_models::ActivityInput},
    jwt_auth,
    models::{Activities, Pool},
    schema::{
        activities::*,
        gym::*,
    },
};
use actix_web::{post, HttpMessage};
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;

#[post("/activities")]
pub async fn create_activity(
    req: HttpRequest,
    activity: web::Json<ActivityInput>,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let user_found_id = ext.get::<uuid::Uuid>().unwrap().clone();
    info!(
        "user_id: {:?} is create activity {:?}",
        user_found_id, activity
    );

    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(activity, user_found_id, conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(activity) => Ok(HttpResponse::Ok().json(activity)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(
    activity: web::Json<ActivityInput>,
    user_has_id: uuid::Uuid,
    conn: &mut PgConnection,
) -> Result<Activities, crate::errors::ServiceError> {
    use crate::schema::activities::dsl::*;
    let same_name_activity_exist = activities
        .select(id)
        .filter(name.eq(&activity.name))
        .filter(gym_id.eq(&activity.gym_id))
        .execute(conn)?;
    if same_name_activity_exist > 0 {
        return Err(crate::errors::ServiceError::BadRequest(
            "Activity name already exist".to_string(),
        ));
    }
    let gym_exist_on_user = query_find_exist_gym(&activity, conn)?;
    debug!("gym_exist_on_user: {:?}", gym_exist_on_user);
    if !gym_exist_on_user {
        return Err(crate::errors::ServiceError::BadRequest(
            "User has not gym on him".to_string(),
        ));
    }

    let result = diesel::insert_into(activities)
        .values(Activities {
            id: uuid::Uuid::new_v4().to_string(),
            name: activity.name.clone(),
            gym_id: activity.gym_id.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
        .get_result(conn);
    match result {
        Ok(activity) => Ok(activity),
        Err(e) => Err(crate::errors::ServiceError::BadRequest(e.to_string())),
    }
}

fn query_find_exist_gym(
    activity: &ActivityInput,
    conn: &mut PgConnection,
) -> Result<bool, crate::errors::ServiceError> {
    use crate::schema::gym::dsl::*;
    let gym_exist_on_user = gym
        .select(id)
        .filter(id.eq(&activity.gym_id))
        .execute(conn)?;

    match gym_exist_on_user {
        1 => Ok(true),
        _ => Ok(false),
    }
}
