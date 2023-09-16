use actix_web::{get, HttpRequest, HttpResponse, web};
use diesel::prelude::*;
use crate::jwt_auth;
use crate::models::{Pool, User};

#[get("/me")]
pub async fn get_user( req: HttpRequest,
                       pool: web::Data<Pool>,
                       jwt: jwt_auth::JwtMiddleware) -> Result<HttpResponse, actix_web::Error> {
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query_get_user(jwt.user_id.to_string(), conn)
    }).await.map_err(actix_web::error::ErrorInternalServerError)?;
    return match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e)),
    }
}

fn query_get_user(user_id: String,conn: &mut PgConnection) -> Result<User, crate::errors::ServiceError> {
    use crate::schema::users::dsl::*;
    let res = users::select(users, User::as_select() ).filter(id.eq(user_id)).limit(1).get_result::<User>(conn)?;
    Ok(res)
}