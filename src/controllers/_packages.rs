use std::sync::mpsc::{self};

use actix_rt::Arbiter;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    controllers::response::response,
    services::_packages::{Package, Packages},
};

use super::response::Response;

pub struct PackagesController;

#[derive(Serialize, ToSchema)]
pub struct PackagesResponse {
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

// Função separada com anotação utoipa
#[utoipa::path(
    get,
    path = "/api/v1/packages",
    responses(
        (status = 200, description = "Lista de pacotes instalados", body = PackagesResponse),
        (status = 500, description = "Erro interno do servidor")
    ),
    tag = "packages"
)]
pub async fn get_packages() -> impl Responder {
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

    drop(tx);

    while let Some(v) = rx.recv().ok() {
        installed_packages.push(v);
    }
    response(PackagesResponse::new(installed_packages))
}

impl PackagesController {
    pub async fn index() -> impl Responder {
        get_packages().await
    }
}