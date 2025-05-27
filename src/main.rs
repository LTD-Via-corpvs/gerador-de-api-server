use std::env;

use actix_web::{web, App, HttpServer};

mod controllers;
mod routes;
mod services;

use routes::register as api_route;
use tracing_actix_web::TracingLogger;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/api/v1").configure(api_route))
            .default_service(web::route().to(crate::controllers::handlers::page_not_found))
    });
    server = server.bind("127.0.0.1:3000")?;
    server.run().await
}
