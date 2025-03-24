use actix_web::{
    HttpResponse, delete, get, patch, post, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    mentor::{CreateMentor, Mentor, MentorUpdate},
};
use macros::handler_implementation;
use service::mentor::MentorServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

use super::MentorAuthResponse;
use crate::common::{ApiError, ValidationError, middleware::auth::AuthEntity};

handler_implementation! {
    MentorHandler as Implemented {
        ///
        ///
        ///
        #[openapi(
            operation_id = "register_mentor",
            request_body(
                description = "",
                content = CreateMentor
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 201, description = "", body = MentorAuthResponse),
                (status = 409, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/register")]
        register(
            mentor_service: Data<MentorServiceDependency>,
            Validated(Json(body)): Validated<Json<CreateMentor>>
        ) -> HttpResponse {
            let resp: MentorAuthResponse = mentor_service
                .register(body)
                .await?
                .into();
            HttpResponse::Created().json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "mentor_login",
            request_body(
                description = "",
                content = LoginRequest
            ),
            responses(
                (status = 200, description = "", body = MentorAuthResponse),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/login")]
        login(
            mentor_service: Data<MentorServiceDependency>,
            Validated(Json(body)): Validated<Json<LoginRequest>>,
        ) -> Json<MentorAuthResponse> {
            let res = mentor_service
                .login(body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_current_mentor",
            security(
                ("mentor" = []),
            ),
            responses(
                (status = 200, description = "", body = Mentor),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("/me")]
        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Mentor> {
            let mentor: Mentor = entity
                .into_inner()
                .try_into()?;
            Json(mentor)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_current_mentor",
            request_body(
                description = "",
                content = MentorUpdate
            ),
            security(
                ("mentor" = []),
            ),
            responses(
                (status = 200, description = "", body = Mentor),
                (status = 409, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/me")]
        update_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Validated(Json(body)): Validated<Json<MentorUpdate>>,
        ) -> Json<Mentor> {
            let mentor: Mentor = entity
                .into_inner()
                .try_into()?;

            if body.username.as_ref().unwrap_or(&mentor.username) == &mentor.username
                && body.name.as_ref().unwrap_or(&mentor.name) == &mentor.name
                && body.surname.as_ref().unwrap_or(&mentor.surname) == &mentor.surname
                && body.bio.as_ref().unwrap_or(&mentor.bio) == &mentor.bio
            {
                return Ok(Json(mentor));
            }

            let res = mentor_service
                .update_by_id(mentor.id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "change_current_mentor_password",
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("mentor" = []),
            ),
            responses(
                (status = 200, description = "", body = MentorAuthResponse),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/me/password")]
        change_password_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<MentorAuthResponse> {
            let mentor: Mentor = entity
                .into_inner()
                .try_into()?;
            let res = mentor_service
                .change_password_by_id(mentor.id, body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_current_mentor",
            security(
                ("mentor" = []),
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
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
        ) -> HttpResponse {
            let mentor: Mentor = entity
                .into_inner()
                .try_into()?;
            mentor_service
                .delete_by_id(mentor.id)
                .await?;
            HttpResponse::NoContent().into()
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_mentor_by_id",
            params(
                ("mentor_id" = Ulid, description = "")
            ),
            security(
                ("organizator" = []),
                ("mentor" = []),
                ("participant" = [])
            ),
            responses(
                (status = 200, description = "", body = Mentor),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{mentor_id}")]
        get_by_id(
            mentor_service: Data<MentorServiceDependency>,
            Path(mentor_id): Path<Ulid>,
        ) -> Json<Mentor> {
            let res = mentor_service
                .get_by_id(mentor_id)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_mentor_by_id",
            params(
                ("mentor_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = MentorUpdate
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Mentor),
                (status = 409, description = "", body = ApiError),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/{mentor_id}")]
        update_by_id(
            mentor_service: Data<MentorServiceDependency>,
            Path(mentor_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<MentorUpdate>>,
        ) -> Json<Mentor> {
            let res = mentor_service
                .update_by_id(mentor_id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "change_mentor_password_by_id",
            params(
                ("mentor_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = MentorAuthResponse),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/{mentor_id}/password")]
        change_password_by_id(
            mentor_service: Data<MentorServiceDependency>,
            Path(mentor_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<MentorAuthResponse> {
            let res = mentor_service
                .change_password_by_id(mentor_id, body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_mentor_by_id",
            params(
                ("mentor_id" = Ulid, description = "")
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
        #[delete("/{mentor_id}")]
        delete_by_id(
            mentor_service: Data<MentorServiceDependency>,
            Path(mentor_id): Path<Ulid>,
        ) -> HttpResponse {
            mentor_service
                .delete_by_id(mentor_id)
                .await?;
            HttpResponse::NoContent().into()
        }
    }
}
