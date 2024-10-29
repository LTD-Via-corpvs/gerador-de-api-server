pub mod _packages;
pub mod _project;

mod response {
    use actix_web::{http::StatusCode, Error, HttpResponse};
    pub trait Response {
        fn to_string(&self) -> String;
    }

    pub fn response_with_status<S: Response>(
        body: S,
        status: StatusCode,
    ) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::build(status)
            .content_type("application/json")
            .body(body.to_string()))
    }

    pub fn response<S: Response>(data: S) -> Result<HttpResponse, Error> {
        response_with_status(data, StatusCode::OK)
    }
}

pub mod handlers {
    use actix_web::{http::StatusCode, Error, HttpResponse};
    use serde::Serialize;

    use super::response::Response;

    #[derive(Serialize)]
    pub struct ErrorResponse {
        error: String,
    }

    impl ErrorResponse {
        pub fn new(error: &str) -> Self {
            Self {
                error: error.to_string(),
            }
        }
    }

    impl Response for ErrorResponse {
        fn to_string(&self) -> String {
            serde_json::to_string(&self).unwrap()
        }
    }

    pub async fn page_not_found() -> Result<HttpResponse, Error> {
        super::response::response_with_status(
            ErrorResponse::new("content not found"),
            StatusCode::NOT_FOUND,
        )
    }
    
    pub fn bad_request(message: &str) -> Result<HttpResponse, Error> {
        super::response::response_with_status(
            ErrorResponse::new(message),
            StatusCode::BAD_REQUEST,
        )
    }
}
