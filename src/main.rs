use actix_web::{
    middleware::{self, Logger},
    web, App, HttpServer,
};
mod db;
pub mod error;
pub mod models;
pub mod routes;
mod schema;
pub mod token;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Server running on localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("content-type", "application/json")))
            .service(
                web::scope("/api")
                    .configure(routes::api::register::config)
                    .configure(routes::api::login::config)
                    .configure(routes::api::notes::config),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
