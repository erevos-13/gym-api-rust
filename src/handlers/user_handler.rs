use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::models::{PasswordUsers, Pool, User};
lazy_static! {
    static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
}
#[derive(Deserialize, Clone, Queryable, Debug, Validate)]
pub struct UserRegister {
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        )
    )]
    pub password: String,
}
fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }
    if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
        Ok(())
    } else {
        return Err(ValidationError::new("Password Validation Failed"));
    }
}
pub async fn register_user(
    register_user: web::Json<UserRegister>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    match register_user.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(actix_web::error::ErrorBadRequest(e));
        }
    };

    let uuid = uuid::Uuid::new_v4();
    let user_register_model = User {
        id: uuid.to_string(),
        username: register_user.username.clone(),
        email: register_user.email.clone(),
        age: register_user.age,
        first_name: register_user.first_name.clone(),
        last_name: register_user.last_name.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let result = web::block(move || {
        let conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        match query(&user_register_model, conn) {
            Ok(user) => {
                match query_password(&user_register_model, register_user.password.clone(), conn) {
                    Ok(_) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => return Err(e.into()),
        }
    })
    .await;

    match result {
        Ok(userCreate) => Ok(HttpResponse::Ok().json(userCreate)),
        Err(e) => Err(e.into()),
    }
}

fn query(user: &User, conn: &mut PgConnection) -> Result<User, crate::errors::ServiceError> {
    use crate::schema::users::dsl::*;
    let user_fount = users
        .select(id)
        .filter(username.eq(&user.username))
        .execute(conn)?;
    if user_fount > 0 {
        return Err(crate::errors::ServiceError::BadRequest(
            "User already exists".into(),
        ));
    }

    let res: User = diesel::insert_into(users)
        .values(user)
        .get_result::<User>(conn)?;
    Ok(res)
}

fn query_password(
    user: &User,
    password_insert: String,
    conn: &mut PgConnection,
) -> Result<PasswordUsers, crate::errors::ServiceError> {
    use crate::schema::password_users::dsl::*;
    let password_user = PasswordUsers {
        id: 0,
        user_id: user.id.clone(),
        password: password_insert,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let res: PasswordUsers = diesel::insert_into(password_users)
        .values(password_user)
        .get_result::<PasswordUsers>(conn)?;
    Ok(res)
}
