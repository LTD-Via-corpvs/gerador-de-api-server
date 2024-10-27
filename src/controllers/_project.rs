use std::{fs::create_dir_all, path::Path};

use actix_web::{web::Json, Responder};
use serde::{Deserialize, Serialize};

use crate::services::_packages::Packages;

use super::response::{response, Response};

pub struct ProjectController;

#[derive(Serialize, Deserialize)]
pub struct ProjectRequest {
    package_id: u8,
    project_name: String,
}

#[derive(Serialize)]
struct ProjectResponse {
    data: ProjectRequest,
    message: String,
}

impl ProjectResponse {
    fn new(data: ProjectRequest, message: String) -> Self {
        Self { data, message }
    }
}

impl Response for ProjectResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ProjectController {
    pub async fn post(body: Json<ProjectRequest>) -> impl Responder {
        let body = body.into_inner();

        let project_path = Path::new("../build").join(&body.project_name);
        create_dir_all(&project_path).unwrap();

        let package = Packages::find_by_id(body.package_id).get();
        package.init(&project_path).await;

        response(ProjectResponse::new(
            body,
            format!("{} init", package.get_name()).to_string(),
        ))
    }
}
