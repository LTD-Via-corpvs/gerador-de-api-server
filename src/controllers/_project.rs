use actix_web::{web::Json, Responder};
use serde::{Deserialize, Serialize};

use super::response::{response, Response};

pub struct ProjectController;

#[derive(Serialize, Deserialize)]
pub struct ProjectRequest {
    package_id: u8,
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
        response(ProjectResponse::new(
            body,
            "Projeto criado com sucesso!".to_string(),
        ))
    }
}
