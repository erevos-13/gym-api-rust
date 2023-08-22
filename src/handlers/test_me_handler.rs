use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;

use crate::{
    errors::ServiceError,
    models::{Pool, Testme},
};

#[derive(Deserialize, Clone, Queryable, Debug)]
pub struct TestMeInsert {
    id: i32,
    count: i32,
}

pub async fn test_me_import(
    test_me_input: web::Json<TestMeInsert>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let test_me_model = Testme {
        id: test_me_input.id.clone(),
        count: test_me_input.count.clone(),
    };

    web::block(move || {
        let conn = &mut pool.get()?;
        query(test_me_model, conn)
    })
    .await?;
    Ok(HttpResponse::Ok().json("Importing test_me"))
}
fn query(test_me: Testme, conn: &mut PgConnection) -> Result<Testme, crate::errors::ServiceError> {
    use crate::schema::testme::dsl::*;
    let res: Testme = diesel::insert_into(testme)
        .values(&test_me)
        .get_result::<Testme>(conn)?;
    Ok(res)
}
