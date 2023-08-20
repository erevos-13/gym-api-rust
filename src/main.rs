mod errors;
mod handlers;
mod models;
mod schema;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use handlers::user_handler;

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
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/api").service(
                web::resource("/register").route(web::post().to(user_handler::register_user)),
            ))
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
