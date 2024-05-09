use rocket::serde::Serialize;
use serde::Deserialize;

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
