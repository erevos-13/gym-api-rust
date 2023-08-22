use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;

use crate::{
    errors::ServiceError,
    models::{Pool, User},
};
#[derive(Deserialize, Clone, Queryable, Debug)]
pub struct UserRegister {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

pub async fn register_user(
    register_user: web::Json<UserRegister>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let uuid = uuid::Uuid::new_v4();
    let user_register_model = User {
        id: 15,
        username: register_user.username.clone(),
        email: register_user.email.clone(),
        age: register_user.age,
        first_name: register_user.first_name.clone(),
        last_name: register_user.last_name.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    web::block(move || {
        let conn = &mut pool.get()?;
        query(user_register_model, conn)
    })
    .await?;
    Ok(HttpResponse::Ok().json("Registering user"))
}

fn query(user: User, conn: &mut PgConnection) -> Result<User, crate::errors::ServiceError> {
    use crate::schema::users::dsl::*;
    let res: User = diesel::insert_into(users)
        .values(&user)
        .get_result::<User>(conn)?;
    Ok(res)
}
