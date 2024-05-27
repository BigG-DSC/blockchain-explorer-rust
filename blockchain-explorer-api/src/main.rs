mod services;
mod models;

use services::explorer_endpoints::{
    get_status,
};
use actix_web::{HttpServer, App, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_status)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
