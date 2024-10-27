use std::sync::mpsc::{self};

use actix_rt::Arbiter;
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
        let (tx, rx) = mpsc::channel::<Package>();

        let arbiter = Arbiter::new();
        for package_manager in package_managers.iter() {
            let pkg = package_manager.get();
            let tx = tx.clone();
            arbiter.spawn(async move {
                if pkg.is_installed().await {
                    tx.send(pkg).unwrap();
                }
            });
        }
        
        drop(tx); // <- Se não dropar, vai ficar no while infinito
        
        while let Some(v) = rx.recv().ok() {
            installed_packages.push(v);
        }
        response(PackagesResponse::new(installed_packages))
    }
}
