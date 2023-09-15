use crate::models::Slots;
use crate::utils::dates::convert_date;
use crate::{input_model::slots_models::SlotsInput, jwt_auth, models::Pool};
use actix_web::{patch, web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl};

#[patch("/slots/{id}")]
pub async fn update_slots(
    id: web::Path<String>,
    _: HttpRequest,
    slot: web::Json<SlotsInput>,
    pool: web::Data<Pool>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(id.clone(), slot.clone(), conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(slot) => Ok(HttpResponse::Ok().json(slot)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
    }
}
fn query(
    slot_id: String,
    slot: SlotsInput,
    conn: &mut PgConnection,
) -> Result<Slots, crate::errors::ServiceError> {
    use crate::schema::slots::dsl::*;
    let exist = slots.filter(id.eq(&slot_id)).limit(1).load::<Slots>(conn)?;
    if exist.len() == 0 {
        return Err(crate::errors::ServiceError::BadRequest(
            "Slot not exist".to_string(),
        ));
    }

    let res = diesel::update(slots.filter(id.eq(&slot_id)))
        .set((
            start_time
                .eq(&DateTime::<Utc>::from_utc(convert_date(slot.start_time), Utc).naive_utc()),
            end_time.eq(&DateTime::<Utc>::from_utc(convert_date(slot.end_time), Utc).naive_utc()),
            activity_id.eq(&slot.activity_id),
            attendants.eq(&slot.attendants),
        ))
        .get_result::<Slots>(conn)?;
    Ok(res)
}
