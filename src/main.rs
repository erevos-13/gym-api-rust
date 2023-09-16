mod errors;
mod handlers;
mod input_model;
mod jwt_auth;
mod models;
mod schema;
mod token;
mod utils;
mod DTO;

use actix_cors::Cors;
use actix_web::{
    http::header, middleware, web, App, HttpResponse, HttpServer,
};
use diesel::{r2d2::ConnectionManager, PgConnection};

use crate::handlers::{
    activities::*,
    appointments::*,
    gym::*,
    slots::*,
    users::*,
};

#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    info!("Starting server at http://127.0.0.1 and port: 8080");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/register")
                            .route(web::post().to(user_handler::register_user)),
                    )
                    .service(
                        web::resource("/login").route(web::post().to(login_handler::login_user)),
                    )
                    .service(create_gym::create_gym)
                    .service(update_gym::update_gym)
                    .service(get_gym::get_gym)
                    .service(delete_gym::delete_gym)
                    .service(create_activity::create_activity)
                    .service(update_activity::update_activity)
                    .service(get_activities::get_activities)
                    .service(delete_activities::delete_activity)
                    .service(create_slots::create_slots)
                    .service(update_slots::update_slots)
                    .service(delete_slots::delete_slot)
                    .service(get_slots::get_slots)
                    .service(create_appointments::create_appointments)
                    .service(get_appointments::get_appointments)
                    .service(get_appoitments_by_user::get_appointments_by_user)
                    .service(add_user_in_gym::add_user_in_gym)
                    .service(get_user::get_user)
            )
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
