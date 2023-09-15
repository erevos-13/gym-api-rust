use crate::{
    errors::ServiceError,
    models::{Gym, PasswordUsers, Pool, User},
    schema::gym,
    token::signing,
};
use actix_web::{web, HttpResponse};
use diesel::{dsl::exists, prelude::*, select};
use pwhash::{bcrypt, unix};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Queryable, Debug, Validate)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

fn query_login(
    login: Login,
    conn: &mut PgConnection,
) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::password_users::dsl::*;
    use crate::schema::users::dsl::*;
    let user_exist = select(exists(users.filter(username.eq(login.username.clone()))))
        .get_result::<bool>(conn)
        .expect("Error loading users");

    if !user_exist {
        return Err(crate::errors::ServiceError::BadRequest(
            "Authentication fail".to_string(),
        ));
    }
    let user_filter = users::filter(users, username.eq(login.username.clone()))
        .load::<User>(conn)
        .expect("Error loading users");
    let password_of_user: PasswordUsers =
        password_users::filter(password_users, user_id.eq(user_filter[0].id.clone()))
            .get_result(conn)?;

    let user = user_filter[0].clone();
    let password_verify = unix::verify(login.password, &password_of_user.password.clone());
    if password_verify {
        info!("User found: {:?}", user);
        let token = match signing(user.id) {
            Ok(token) => token,
            Err(e) => return Err(crate::errors::ServiceError::BadRequest(e.to_string())),
        };
        Ok(token)
    } else {
        Err(crate::errors::ServiceError::BadRequest(
            "Authentication fail".to_string(),
        ))
    }
}

pub async fn login_user(
    login: web::Json<Login>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let login_input = Login {
        username: login.username.clone(),
        password: login.password.clone(),
    };
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        query_login(login_input, conn)
    })
    .await?;
    match result {
        Ok(token) => {
            let token_response = TokenResponse { token };
            return Ok(HttpResponse::Ok().json(token_response));
        }
        Err(e) => return Err(actix_web::error::ErrorBadRequest(e)),
    };
}

