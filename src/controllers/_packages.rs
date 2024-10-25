use actix_web::Responder;
use serde::Serialize;

use crate::{
    controllers::response::response,
    services::_packages::{Package, Packages},
};

use super::response::Response;

pub struct PackagesController;

#[derive(Serialize)]
struct PackagesResponse {
    packages: Vec<Package>,
}

impl PackagesResponse {
    fn new(packages: Vec<Package>) -> Self {
        Self { packages }
    }
}

impl Response for PackagesResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl PackagesController {
    pub async fn index() -> impl Responder {
        let package_managers = [Packages::BUN, Packages::PNPM, Packages::YARN, Packages::NPM];
        let mut installed_packages = vec![];
        for package_manager in package_managers.iter() {
            let pkg = package_manager.get();
            if pkg.is_installed() {
                installed_packages.push(pkg);
            }
        }
        response(PackagesResponse::new(installed_packages))
    }
}
