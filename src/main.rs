use actix_web::{web, App, HttpServer};

mod controllers;
mod routes;
mod services;

use routes::register as api_route;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/api/v1").configure(api_route))
            .default_service(web::route().to(crate::controllers::handlers::page_not_found))
    });
    server = server.bind("127.0.0.1:3333")?;
    server.run().await
}
