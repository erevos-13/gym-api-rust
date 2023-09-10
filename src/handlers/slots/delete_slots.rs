use crate::{jwt_auth, models::Pool};
use actix_web::{delete, web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl};

#[delete("/slots/{id}")]
pub async fn delete_slot(
    id: web::Path<String>,
    _: HttpRequest,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(id.clone(), conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(slot) => Ok(HttpResponse::Ok().json(slot)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}

fn query(slot_id: String, con: &mut PgConnection) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::slots::dsl::*;
    let slot_found = slots.select(id).filter(id.eq(&slot_id)).execute(con)?;
    if slot_found >= 1 {
        let _res = diesel::delete(slots.filter(id.eq(&slot_id))).execute(con)?;
        return Ok("Slot deleted".to_string());
    }
    return Err(crate::errors::ServiceError::BadRequest(
        "Slot does'n exists".to_string(),
    ));
}
