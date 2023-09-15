use crate::{
    input_model::slots_models::SlotsInput,
    jwt_auth::{self},
    models::{ Pool, Slots},
    utils::dates::convert_date,
};
use actix_web::{post, web, HttpRequest, HttpResponse};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use crate::schema::users_gym::dsl::users_gym;

#[post("/slots")]
pub async fn create_slots(
    req: HttpRequest,
    slots: web::Json<Vec<SlotsInput>>,
    pool: web::Data<Pool>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("jwt: {:?}", jwt);
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(slots.clone(), jwt.user_id.to_string(), conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    info!(
        "result of insert slots: {:?} by user: {:?}",
        result, jwt.user_id
    );
    Ok(HttpResponse::Ok().json(result))
}

fn query(
    slot: Vec<SlotsInput>,
    id_user: String,
    conn: &mut PgConnection,
) -> Result<Vec<Slots>, crate::errors::ServiceError> {
    use crate::schema::slots::dsl::*;
    let start_date_vec = get_start_date(&slot);
    let end_date_vec = get_end_time_date(&slot);
    info!(
        "Start Time Convert: {:?}, End time convert: {:?}",
        start_date_vec, end_date_vec
    );
    let gym_user_id = query_find_gym_user_id(id_user, conn)?;
    let exist = slots
        .select(slots::all_columns())
        .filter(start_time.eq_any(&start_date_vec))
        .or_filter(end_time.eq_any(&end_date_vec))
        .filter(gym_id.eq(&gym_user_id))
        .limit(1)
        .load::<Slots>(conn)?;

    if exist.len() > 0 {
        warn!("exist: {:?}", exist);
        warn!("exist: {:?}", exist.len());
        return Err(crate::errors::ServiceError::BadRequest(format!(
            "Slot already exist with start_time: {:?} and end_time: {:?}",
            exist[0].start_time, exist[0].end_time
        )));
    }

    let slots_vec = create_slot_from_array(slot, gym_user_id);
    let res = diesel::insert_into(slots)
        .values(&slots_vec)
        .get_results::<Slots>(conn)?;
    Ok(res)
}

fn create_slot_from_array(slots: Vec<SlotsInput>, gym_id: String) -> Vec<Slots> {
    let mut slots_vec: Vec<Slots> = Vec::new();
    for slot in slots {
        let start_time_convert = convert_date(slot.start_time.clone());
        let end_time_convert = convert_date(slot.end_time.clone());
        let slot = Slots {
            id: uuid::Uuid::new_v4().to_string(),
            gym_id: gym_id.clone(),
            attendants: slot.attendants,
            start_time: DateTime::<Utc>::from_utc(start_time_convert, Utc).naive_utc(),
            end_time: DateTime::<Utc>::from_utc(end_time_convert, Utc).naive_utc(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            activity_id: slot.activity_id.clone(),
        };
        slots_vec.push(slot);
    }
    slots_vec
}
fn get_start_date(slots: &Vec<SlotsInput>) -> Vec<NaiveDateTime> {
    let mut start_date_vec: Vec<NaiveDateTime> = Vec::new();
    for slot in slots {
        let start_time_convert = convert_date(slot.start_time.clone());
        start_date_vec.push(start_time_convert);
    }
    start_date_vec
}

fn get_end_time_date(slots: &Vec<SlotsInput>) -> Vec<NaiveDateTime> {
    let mut end_date_vec: Vec<NaiveDateTime> = Vec::new();
    for slot in slots {
        let end_time_convert = convert_date(slot.end_time.clone());
        end_date_vec.push(end_time_convert);
    }
    end_date_vec
}

fn query_find_gym_user_id(id_user:String, conn: &mut PgConnection) -> Result<String,crate::errors::ServiceError>{
    use crate::schema::users_gym::dsl::*;
    let res = users_gym
        .select(gym_id)
        .filter(user_id.eq(id_user))
        .limit(1)
        .get_result::<String>(conn)?;
    Ok(res)
}