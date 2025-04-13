use actix_web::{
    HttpResponse, delete, get, patch, post, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    user::{CreateUser, User, UserUpdate},
};
use macros::handler_implementation;
use service::user::UserServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

use super::{
    UserAuthResponse, UserHandler, UserHandlerHelper, UserHandlerResult,
};
use crate::common::{ApiError, ValidationError};

handler_implementation! {
    UserHandler as Implemented {
        ///
        ///
        ///
        #[openapi(
            operation_id = "register_user",
            request_body(
                description = "",
                content = CreateUser
            ),
            responses(
                (status = 201, description = "", body = UserAuthResponse),
                (status = 409, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/register")]
        register(
            user_service: Data<UserServiceDependency>,
            Validated(Json(body)): Validated<Json<CreateUser>>
        ) -> HttpResponse {
            let resp: UserAuthResponse = user_service
                .register(body)
                .await?
                .into();
            HttpResponse::Created().json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "user_login",
            request_body(
                description = "",
                content = LoginRequest
            ),
            responses(
                (status = 200, description = "", body = UserAuthResponse),
                (status = 401, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/login")]
        login(
            user_service: Data<UserServiceDependency>,
            Validated(Json(body)): Validated<Json<LoginRequest>>,
        ) -> Json<UserAuthResponse> {
            let res = user_service
                .login(body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_current_user",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = User),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("/me")]
        get_current(
            user: ReqData<User>,
        ) -> Json<User> {
            let user: User = user.into_inner();
            Json(user)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_current_user",
            request_body(
                description = "",
                content = UserUpdate
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = User),
                (status = 409, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/me")]
        update_current(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            Validated(Json(body)): Validated<Json<UserUpdate>>,
        ) -> Json<User> {
            let user: User = user.into_inner();

            if body.username.as_ref().unwrap_or(&user.username) == &user.username {
                return Ok(Json(user));
            }

            let res = user_service
                .update_by_id(user.id, body, true)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "change_current_user_password",
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = UserAuthResponse),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/me/password")]
        change_password_current(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<UserAuthResponse> {
            let user: User = user.into_inner();
            let res = user_service
                .change_password_by_id(user.id, body, true)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_current_user",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/me")]
        delete_current(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse {
            let user: User = user.into_inner();
            user_service
                .delete_by_id(user.id)
                .await?;
            HttpResponse::NoContent().finish()
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_user_by_id",
            params(
                ("user_id" = Ulid, description = "")
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = User),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{user_id}")]
        get_by_id(
            user_service: Data<UserServiceDependency>,
            Path(user_id): Path<Ulid>,
        ) -> Json<User> {
            let res = user_service
                .get_by_id(user_id)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_user_by_id",
            params(
                ("user_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = UserUpdate
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = User),
                (status = 409, description = "", body = ApiError),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/{user_id}")]
        update_by_id(
            user_service: Data<UserServiceDependency>,
            Path(user_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<UserUpdate>>,
        ) -> Json<User> {
            let res = user_service
                .update_by_id(user_id, body, false)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "change_user_password_by_id",
            params(
                ("user_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = UserAuthResponse),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/{user_id}/password")]
        change_password_by_id(
            user_service: Data<UserServiceDependency>,
            Path(user_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<UserAuthResponse> {
            let res = user_service
                .change_password_by_id(user_id, body, false)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_user_by_id",
            params(
                ("user_id" = Ulid, description = "")
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
            ),
        )]
        #[delete("/{user_id}")]
        delete_by_id(
            user_service: Data<UserServiceDependency>,
            Path(user_id): Path<Ulid>,
        ) -> HttpResponse {
            user_service
                .delete_by_id(user_id)
                .await?;
            HttpResponse::NoContent().finish()
        }
    }
}
