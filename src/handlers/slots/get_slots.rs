use actix_web::{get, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::{
    input_model::gym_input_model::ResponseDTO,
    jwt_auth,
    models::{Pool, Slots},
};
#[get("/slots")]
pub async fn get_slots(
    _: HttpRequest,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(slots) => Ok(HttpResponse::Ok().json(slots)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(conn: &mut PgConnection) -> Result<ResponseDTO<Slots>, crate::errors::ServiceError> {
    use crate::schema::slots::dsl::*;
    let res = slots.load::<Slots>(conn)?;
    let total_Count = slots.count().get_result::<i64>(conn)?;

    Ok(ResponseDTO {
        data: res,
        total: total_Count as usize,
    })
}
