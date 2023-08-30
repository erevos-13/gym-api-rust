use crate::{
    models::{PasswordUsers, Pool, User},
    token::signing,
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use pwhash::{bcrypt, unix};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Queryable, Debug, Validate)]
pub struct Login {
    pub username: String,
    pub password: String,
}

fn query_login(
    login: Login,
    conn: &mut PgConnection,
) -> Result<String, crate::errors::ServiceError> {
    use crate::schema::password_users::dsl::*;
    use crate::schema::users::dsl::*;

    let user_found = password_users::inner_join(password_users, users)
        .filter(username.eq(login.username))
        .load::<(crate::models::PasswordUsers, crate::models::User)>(conn)
        .expect("Error loading users");

    let user = user_found[0].1.clone();
    dbg!(user_found[0].0.password.clone());
    let password_verify = unix::verify(login.password, &user_found[0].0.password.clone());
    if password_verify {
        println!("User found: {:?}", user);
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
    Ok(HttpResponse::Ok().json(result))
}
