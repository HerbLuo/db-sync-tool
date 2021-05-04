use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::http::Status;
use rocket::response::{Responder, Response};
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

impl<'r, R: Responder<'r>> Responder<'r> for WithStatus<R> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Response::build_from(self.1.respond_to(req)?)
            .status(self.0)
            .ok()
    }
}

impl<'r> Responder<'r> for ZzErrors {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Response::build_from(fail!(crate::ec::ServerError, self).respond_to(req)?)
            .status(Status::InternalServerError)
            .ok()
    }
}

#[allow(dead_code)]
pub type HttpErrorData = WithStatus<Json<Data<HttpError>>>;

#[allow(dead_code)]
pub type JsonResult<T> = Result<Json<Data<T>>, HttpErrorData>;

pub type ZzJsonResult<T> = Result<Json<Data<T>>, ZzErrors>;
