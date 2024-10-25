use actix_web::web::{get, post, resource, ServiceConfig};

use crate::controllers::{_packages::PackagesController, _project::ProjectController};

pub fn register(config: &mut ServiceConfig) {
    config.service(resource("/packages").route(get().to(PackagesController::index)));
    config.service(resource("/project").route(post().to(ProjectController::post)));
}
