use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};

use crate::{
    input_model::slots_models::SlotsInput,
    jwt_auth::{self, JwtMiddleware},
    models::{Gym, Pool},
};

#[post("/slots")]
pub async fn create_slots(
    req: HttpRequest,
    slots: web::Json<SlotsInput>,
    pool: web::Data<Pool>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    debug!("ext: {:?}", ext.get::<JwtMiddleware>());
    debug!("jwt: {:?}", jwt);
    let gym_id = ext.get::<uuid::Uuid>().unwrap();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    Ok(HttpResponse::Ok().json("create_slots"))
}
