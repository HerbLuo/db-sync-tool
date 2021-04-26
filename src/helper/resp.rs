use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::request::Request;

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

pub type HttpErrorData = WithStatus<Json<Data<HttpError>>>;

pub type JsonResult<T> = Result<Json<Data<T>>, HttpErrorData>;
