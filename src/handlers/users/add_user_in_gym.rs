use actix_web::{HttpRequest, HttpResponse, post, web};
use diesel::PgConnection;
use crate::jwt_auth;
use crate::models::{Pool, UsersGyms};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users_gym::dsl::users_gym;

#[derive(Deserialize, Serialize, Clone, Queryable, Debug)]
pub struct AddUserInGym {
    pub gym_id: String,
}

#[post("/add_user_in_gym")]
pub async fn add_user_in_gym(
    gym_input: web::Json<AddUserInGym>,
    req: HttpRequest,
    pool: web::Data<Pool>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query(jwt.user_id.to_string().clone(),gym_input.gym_id.clone(), conn)
    }).await.map_err(actix_web::error::ErrorInternalServerError)?;
    return match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e)),
    }
}


fn query(jwt_user_id: String, add_gym_id: String,conn: &mut PgConnection ) -> Result<String, crate::errors::ServiceError> {
    let user_gym = UsersGyms {
        user_id: jwt_user_id.clone(),
        gym_id: add_gym_id.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    let res = diesel::insert_into(users_gym)
        .values(user_gym)
        .get_result::<UsersGyms>(conn)?;

    Ok("User update successfully".to_string())

}