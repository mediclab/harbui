use crate::types::ImageManifest;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CountResponse {
    pub count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConfigResponse {
    pub registry_domain: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ImageManifestResponse {
    pub image: String,
    pub tag: String,
    pub manifests: Vec<ImageManifest>,
}

pub type ApiResponse<T> = Result<ApiAnswer<T>, ApiError>;

#[derive(Clone, Debug)]
pub struct ApiAnswer<T> {
    pub json: Json<T>,
    pub status: Status,
}

impl<T> ApiAnswer<T> {
    pub fn success(object: T) -> ApiResponse<T>
    where
        T: Serialize,
    {
        Ok(ApiAnswer {
            json: Json(object),
            status: Status::Ok,
        })
    }
}

impl<'r, T> Responder<'r, 'r> for ApiAnswer<T>
where
    T: Serialize,
{
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

pub struct ApiError {
    pub status: Status,
    pub message: String,
}

impl ApiError {
    pub fn unprocessable(message: &str) -> Self {
        Self {
            status: Status::UnprocessableEntity,
            message: message.to_owned(),
        }
    }

    pub fn not_found(message: &str) -> Self {
        Self {
            status: Status::NotFound,
            message: message.to_owned(),
        }
    }
}
impl<'r> Responder<'r, 'r> for ApiError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.message.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
