use crate::{
    input_model::gym_input_model::GymRegister,
    jwt_auth,
    models::{Gym, Pool},
};
use actix_web::{post};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;
#[post("/gym")]
pub async fn create_gym(
    req: HttpRequest,
    gym_register: web::Json<GymRegister>,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let uuid = uuid::Uuid::new_v4();
    let gym_new = Gym {
        id: uuid.to_string(),
        name: gym_register.name.clone(),
        address: gym_register.address.clone(),
        postal_code: gym_register.postal_code.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(&gym_new, conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(gym) => Ok(HttpResponse::Ok().json(gym)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(gym_create: &Gym, conn: &mut PgConnection) -> Result<Gym, crate::errors::ServiceError> {
    use crate::schema::gym::dsl::*;
    let gym_found = gym
        .select(name)
        .filter(name.eq(&gym_create.name))
        .execute(conn)?;
    if gym_found == 0 {
        let res: Gym = diesel::insert_into(gym)
            .values(gym_create)
            .get_result::<Gym>(conn)?;
        return Ok(res);
    }
    return Err(crate::errors::ServiceError::AlreadyExists(
        "Gym already exists".to_string(),
    ));
}
