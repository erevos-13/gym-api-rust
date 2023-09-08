use crate::{
    jwt_auth,
    models::{Gym, Pool},
};
use actix_web::HttpMessage;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use diesel::{expression, prelude::*};
use serde::Deserialize;
use validator::Validate;
#[derive(Deserialize, Clone, Queryable, Debug, Validate)]
pub struct GymRegister {
    pub name: String,
    pub address: String,
    pub postal_code: i32,
}

pub async fn create_gym(
    req: HttpRequest,
    gym_register: web::Json<GymRegister>,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let uuid = uuid::Uuid::new_v4();
    let gym_new = Gym {
        id: uuid.to_string(),
        name: gym_register.name.clone(),
        address: gym_register.address.clone(),
        postal_code: gym_register.postal_code.clone(),
        user_id: user_id.to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        match query(&gym_new, conn) {
            Ok(gym) => HttpResponse::Ok().json(gym),
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        }
    })
    .await?
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
    return Err(crate::errors::ServiceError::BadRequest(
        "Gym already exists".to_string(),
    ));
}
