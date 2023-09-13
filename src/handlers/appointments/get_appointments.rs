use actix_web::{get, HttpRequest, HttpResponse, web};
use crate::input_model::appointment_model::AppointmentsInput;
use crate::jwt_auth;
use crate::models::Pool;

#[get("/appointments")]
pub async fn get_appointments(
    req: HttpRequest,
    appointment_input: web::Json<AppointmentsInput>,
    pool: web::Data<Pool>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("get_appointments"))
}
