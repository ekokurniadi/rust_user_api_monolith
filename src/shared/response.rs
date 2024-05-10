use rocket::serde::Serialize;
use serde::Deserialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseBody<T = Option<()>> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseBodyWithPagination<T = Option<()>> {
    pub status: u16,
    pub message: String,
    pub meta: Meta,
    pub data: T,
}

#[derive(Serialize, Default, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Meta {
    pub total_data: i64,
    pub per_page: i64,
    pub page: i64,
}

#[derive(Deserialize, FromForm, Debug,Clone)]
#[serde(crate = "rocket::serde")]
pub struct RequestPaginationParam {
    pub page: i64,
    pub limit: i64,
}


#[derive(Responder)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(Value),
    #[response(status = 400)]
    BadRequest(Value),
    #[response(status = 401)]
    Unauthorized(Value),
    #[response(status = 404)]
    NotFound(Value),
    #[response(status = 500)]
    InternalServerError(Value),
}