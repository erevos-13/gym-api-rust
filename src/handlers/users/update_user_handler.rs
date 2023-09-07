use actix_web::{web, HttpResponse};

use crate::models::Pool;

use super::user_handler::UserRegister;

pub async fn update_user(
    register_user: web::Json<UserRegister>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json("update user"))
}
