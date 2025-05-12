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
use tracing::instrument;
use ulid::Ulid;

use super::{
    UserAuthResponse, UserHandler, UserHandlerHelper, UserHandlerResult,
};
use crate::common::{ApiError, ValidationError, openapi};

handler_implementation! {
    UserHandler as UserHandlerImpl {
        ///
        ///
        ///
        #[openapi(
            security(
                ("organizer" = []),
            ),
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
        #[instrument(skip_all, name = "UserHandler::register_user")]
        async fn register_user(
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
        #[instrument(skip_all, name = "ReviewHandler::user_login")]
        async fn user_login(
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
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = User),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("/me")]
        #[instrument(skip_all, name = "ReviewHandler::get_current_user")]
        async fn get_current_user(
            user: ReqData<User>,
        ) -> Json<User> {
            let user: User = user.into_inner();
            Json(user)
        }

        ///
        ///
        ///
        #[openapi(
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            request_body(
                description = "",
                content = UserUpdate
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
        #[instrument(skip_all, name = "ReviewHandler::update_current_user")]
        async fn update_current_user(
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
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            responses(
                (status = 200, description = "", body = UserAuthResponse),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/me/password")]
        #[instrument(skip_all, name = "ReviewHandler::change_current_user_password")]
        async fn change_current_user_password(
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
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/me")]
        #[instrument(skip_all, name = "ReviewHandler::delete_current_user")]
        async fn delete_current_user(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse {
            let user: User = user.into_inner();
            user_service
                .delete_by_id(user.id, false)
                .await?;
            HttpResponse::NoContent().finish()
        }

        ///
        ///
        ///
        #[openapi(
            params(
                ("user_id" = Ulid, description = "")
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = User),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{user_id}")]
        #[instrument(skip_all, name = "ReviewHandler::get_user_by_id")]
        async fn get_user_by_id(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            Path(user_id): Path<Ulid>,
        ) -> Json<User> {
            let user: User = user.into_inner();
            let res = if user_id == user.id {
                user
            } else {
                user_service
                .get_by_id(user_id)
                .await?
            };
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            params(
                ("user_id" = Ulid, description = "")
            ),
            security(
                ("organizer" = []),
            ),
            request_body(
                description = "",
                content = UserUpdate
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
        #[instrument(skip_all, name = "ReviewHandler::update_user_by_id")]
        async fn update_user_by_id(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            Path(user_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<UserUpdate>>,
        ) -> Json<User> {
            let res = user_service
                .update_by_id(user_id, body, user_id == user.id)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            params(
                ("user_id" = Ulid, description = "")
            ),
            security(
                ("organizer" = []),
            ),
            request_body(
                description = "",
                content = PasswordChangeRequest
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
        #[instrument(skip_all, name = "ReviewHandler::change_user_password_by_id")]
        async fn change_user_password_by_id(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            Path(user_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<UserAuthResponse> {
            let res = user_service
                .change_password_by_id(user_id, body, user_id == user.id)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            params(
                ("user_id" = Ulid, description = "")
            ),
            security(
                ("organizer" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
            ),
        )]
        #[delete("/{user_id}")]
        #[instrument(skip_all, name = "ReviewHandler::delete_user_by_id")]
        async fn delete_user_by_id(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            Path(user_id): Path<Ulid>,
        ) -> HttpResponse {
            user_service
                .delete_by_id(user_id, user_id != user.id)
                .await?;
            HttpResponse::NoContent().finish()
        }
    }
}
