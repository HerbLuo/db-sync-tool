use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use rocket::request::Request;

use crate::types::ZzErrors;

#[derive(Debug, Serialize)]
pub struct Data<T> {
    pub ok: u8,
    // 0 false, 1 true
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct HttpError {
    pub code: u16,
    pub serial: String,
    pub tip: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithStatus<R>(pub Status, pub R);

impl<'r, R: Responder<'r, 'static>> Responder<'r, 'static> for WithStatus<R> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.1.respond_to(request)?)
            .status(self.0)
            .ok()
    }
}


impl<'r> Responder<'r, 'static> for ZzErrors {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(fail!(crate::ec::ServerError, self).respond_to(request)?)
            .status(Status::InternalServerError)
            .ok()
    }
}

#[allow(dead_code)]
pub type HttpErrorData = WithStatus<Json<Data<HttpError>>>;

#[allow(dead_code)]
pub type JsonResult<T> = Result<Json<Data<T>>, HttpErrorData>;

pub type ZzJsonResult<T> = Result<Json<Data<T>>, ZzErrors>;
