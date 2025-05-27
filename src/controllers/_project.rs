use std::{fs::create_dir_all, path::Path};

use actix_web::{web::{self, Json}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::{_packages::Packages, _project::Project};

use super::{handlers::bad_request, response::{response, Response}};

pub struct ProjectController;

#[derive(Serialize, Deserialize)]
pub struct ProjectRequest {
    package_id: u8,
    project_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectRouteModelRequest {
    model: String,
    file: String,
    route: String,
    junction_table: Option<String>
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
        let mut body = body.into_inner();
        
        body.project_name = body.project_name.to_lowercase().replace(" ", "-");

        let project_path = Path::new("../build").join(&body.project_name);
        if project_path.is_dir() {
            return bad_request("Já existe um projeto com esse nome")
        }
        create_dir_all(&project_path).unwrap();

        let package = Packages::find_by_id(body.package_id).get();
        package.init(&project_path).await;

        response(ProjectResponse::new(
            body,
            format!("{} init", package.get_name()).to_string(),
        ))
    }
    
    pub async fn post_route_model(project_name: web::Path<String>, body: Json<ProjectRouteModelRequest>) -> actix_web::Result<impl Responder> {
        let project_name = project_name.to_lowercase().replace(" ", "-");

        let project_path = Path::new("../build").join(&project_name);
        if !project_path.is_dir() {
            return bad_request("Não existe um projeto com esse nome")
        }

        let ProjectRouteModelRequest { model, file, junction_table, route } = body.into_inner();
        let project = Project::read(project_path.join("uran.toml").to_str().unwrap());
        let model_capitalized = model.chars().next().unwrap_or_default().to_uppercase().to_string() + &model.chars().skip(1).collect::<String>().to_lowercase();
        let file_lowercase = file.to_lowercase();
        let route_lowercase = route.to_lowercase();

        project.generate_model(&model_capitalized, &file_lowercase, &junction_table.unwrap_or_else(|| "".to_string()))?;
        project.generate_controller(&model_capitalized, &file_lowercase)?;
        project.generate_route(&model_capitalized, &file_lowercase, &route_lowercase)?;

        Ok(HttpResponse::Ok().json(json!({ "data": project.data })))
    }
}
