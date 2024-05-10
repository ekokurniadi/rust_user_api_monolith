use crate::{
    models::users_model::{LoginUser, NewUser},
    services::users::{IUserRepository, UserService},
    shared::{
        middleware::{create_jwt, JWT},
        response::{
            Meta, NetworkResponse, RequestPaginationParam, ResponseBody, ResponseBodyWithPagination,
        },
    },
};
use rocket::{get, post, serde::json::Json, State};

#[get("/users?<params..>")]
pub async fn get_users(
    user_service: &State<UserService>,
    params: Option<RequestPaginationParam>,
    key: Result<JWT, String>,
) -> NetworkResponse {
    match key {
        Ok(key) => key,
        Err(err) => {
            let response = ResponseBody {
                status: rocket::http::Status::Unauthorized.code,
                message: err,
                data: Some(()),
            };

            return NetworkResponse::Unauthorized(serde_json::json!(&response));
        }
    };

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

            NetworkResponse::Created(serde_json::json!(&response))
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            NetworkResponse::InternalServerError(serde_json::json!(&response))
        }
    }
}

#[post("/users", data = "<user>")]
pub async fn create_user(
    user_service: &State<UserService>,
    user: Json<NewUser>,
    key: Result<JWT, String>,
) -> NetworkResponse {
    match key {
        Ok(key) => key,
        Err(err) => {
            let response = ResponseBody {
                status: rocket::http::Status::Unauthorized.code,
                message: err,
                data: Some(()),
            };

            return NetworkResponse::Unauthorized(serde_json::json!(&response));
        }
    };
    let result = user_service.create_user(user.into_inner()).await;

    match result {
        Ok(res) => {
            let response = ResponseBody {
                status: rocket::http::Status::Ok.code,
                message: "create user successfuly".to_string(),
                data: res,
            };

            NetworkResponse::Created(serde_json::json!(&response))
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            NetworkResponse::InternalServerError(serde_json::json!(&response))
        }
    }
}

#[patch("/users/<user_id>", data = "<user>")]
pub async fn update_user(
    user_service: &State<UserService>,
    user_id: i32,
    user: Json<NewUser>,
    key: Result<JWT, String>,
) -> NetworkResponse {
    match key {
        Ok(key) => key,
        Err(err) => {
            let response = ResponseBody {
                status: rocket::http::Status::Unauthorized.code,
                message: err,
                data: Some(()),
            };

            return NetworkResponse::Unauthorized(serde_json::json!(&response));
        }
    };
    let result = user_service.update_user(user_id, user.into_inner()).await;

    match result {
        Ok(res) => {
            let response = ResponseBody {
                status: rocket::http::Status::Ok.code,
                message: "update user successfully".to_string(),
                data: res,
            };

            NetworkResponse::Created(serde_json::json!(&response))
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            NetworkResponse::InternalServerError(serde_json::json!(&response))
        }
    }
}

#[delete("/users/<user_id>")]
pub async fn delete_user(
    user_service: &State<UserService>,
    user_id: i32,
    key: Result<JWT, String>,
) -> NetworkResponse {
    match key {
        Ok(key) => key,
        Err(err) => {
            let response = ResponseBody {
                status: rocket::http::Status::Unauthorized.code,
                message: err,
                data: Some(()),
            };

            return NetworkResponse::Unauthorized(serde_json::json!(&response));
        }
    };
    let result = user_service.delete_user(user_id).await;

    match result {
        Ok(res) => {
            if res {
                let response = ResponseBody {
                    status: rocket::http::Status::Ok.code,
                    message: "delete user successfully".to_string(),
                    data: res,
                };

                NetworkResponse::Created(serde_json::json!(&response))
            } else {
                let response = ResponseBody {
                    status: rocket::http::Status::NotFound.code,
                    message: "not found".to_string(),
                    data: res,
                };

                NetworkResponse::NotFound(serde_json::json!(&response))
            }
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some(()),
            };

            NetworkResponse::InternalServerError(serde_json::json!(&response))
        }
    }
}

#[post("/login", data = "<user>")]
pub async fn login_user(
    user_service: &State<UserService>,
    user: Json<LoginUser>,
) -> NetworkResponse {
    let result = user_service.login(user.into_inner()).await;

    match result {
        Ok(res) => {
            let token = create_jwt(res.id);
            let response = ResponseBody {
                status: rocket::http::Status::Ok.code,
                message: "Login user successfuly".to_string(),
                data: token.unwrap(),
            };

            NetworkResponse::Created(serde_json::json!(&response))
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseBody {
                    status: rocket::http::Status::NotFound.code,
                    message: err.to_string(),
                    data: Some(()),
                };
                return NetworkResponse::NotFound(serde_json::json!(&response));
            }
            _ => {
                let response = ResponseBody {
                    status: rocket::http::Status::InternalServerError.code,
                    message: err.to_string(),
                    data: Some(()),
                };
                return NetworkResponse::InternalServerError(serde_json::json!(&response));
            }
        },
    }
}
