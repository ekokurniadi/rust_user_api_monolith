use crate::{
    models::users_model::NewUser,
    services::users::{IUserRepository, UserService},
    shared::response::{Meta, RequestPaginationParam, ResponseBody, ResponseBodyWithPagination},
};
use rocket::{get, post, serde::json::Json, State};
use serde_json::Value;

#[get("/users?<params..>")]
pub async fn get_users(
    user_service: &State<UserService>,
    params: Option<RequestPaginationParam>,
) -> Json<Value> {
    let mut new_param = RequestPaginationParam { page: 1, limit: 10 };

    if let Some(params) = params {
        new_param = RequestPaginationParam {
            page: params.page,
            limit: params.limit,
        };
    };
    let (result, total) = user_service
        .get_users(new_param.page, new_param.limit)
        .await;

    match result {
        Ok(res) => {
            let meta = Meta {
                total_data: total,
                page: new_param.page,
                per_page: new_param.limit,
            };

            let response = ResponseBodyWithPagination {
                status: rocket::http::Status::Ok.code,
                message: "get users".to_string(),
                meta,
                data: res,
            };

            Json(serde_json::json!(&response))
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            Json(serde_json::json!(&response))
        }
    }
}

#[post("/users", data = "<user>")]
pub async fn create_user(user_service: &State<UserService>, user: Json<NewUser>) -> Json<Value> {
    let result = user_service.create_user(user.into_inner()).await;

    match result {
        Ok(res) => {
            let response = ResponseBody {
                status: rocket::http::Status::Ok.code,
                message: "create user successfuly".to_string(),
                data: res,
            };

            Json(serde_json::json!(&response))
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            Json(serde_json::json!(&response))
        }
    }
}

#[patch("/users/<user_id>", data = "<user>")]
pub async fn update_user(
    user_service: &State<UserService>,
    user_id: i32,
    user: Json<NewUser>,
) -> Json<Value> {
    let result = user_service.update_user(user_id, user.into_inner()).await;

    match result {
        Ok(res) => {
            let response = ResponseBody {
                status: rocket::http::Status::Ok.code,
                message: "update user successfully".to_string(),
                data: res,
            };

            Json(serde_json::json!(&response))
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            Json(serde_json::json!(&response))
        }
    }
}

#[delete("/users/<user_id>")]
pub async fn delete_user(user_service: &State<UserService>, user_id: i32) -> Json<Value> {
    let result = user_service.delete_user(user_id).await;

    match result {
        Ok(res) => {
            if res {
                let response = ResponseBody {
                    status: rocket::http::Status::Ok.code,
                    message: "delete user successfully".to_string(),
                    data: res,
                };

                Json(serde_json::json!(&response))
            } else {
                let response = ResponseBody {
                    status: rocket::http::Status::NotFound.code,
                    message: "not found".to_string(),
                    data: res,
                };

                Json(serde_json::json!(&response))
            }
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            Json(serde_json::json!(&response))
        }
    }
}
