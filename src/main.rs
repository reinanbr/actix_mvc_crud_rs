
mod controllers;

use env_logger;
use controllers::{home_controller::home};
use actix_web::{App,web,middleware::Logger, HttpServer};





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::new("<%s | %b>::[%r] -> [%{User-Agent}i] (%Ts)"))
        .service(
            web::scope("/app")
                .route("/home", web::get().to(home)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}