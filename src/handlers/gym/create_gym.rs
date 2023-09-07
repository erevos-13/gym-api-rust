use crate::{
    jwt_auth,
    models::{Gym, Pool},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::prelude::Queryable;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Clone, Queryable, Debug, Validate)]
pub struct GymRegister {
    pub name: String,
    pub address: String,
    pub postal_code: i32,
}

pub async fn create_gym(
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
        user_id: "1".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    Ok(HttpResponse::Ok().json("gym create"))
}
