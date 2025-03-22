use super::MentorAuthResponse;
use crate::common::{ApiError, middleware::auth::AuthEntity, validate};
use actix_web::{
    HttpResponse, delete, get, patch, post, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    mentor::{CreateMentor, Mentor, MentorUpdate},
};
use macros::handler_implementation;
use service::mentor::MentorServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

handler_implementation! {
    MentorHandler as Implemented {
        #[openapi(
            tag = "Mentors",
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
                (status = 400, description = "", body = ApiError),
            ),
        )]
        #[post("/register")]
        ///
        ///
        ///
        register(
            mentor_service: Data<MentorServiceDependency>,
            Json(body): Json<CreateMentor>
        ) -> HttpResponse {
            validate(&body)?;
            let resp: MentorAuthResponse = mentor_service
                .register(body)
                .await?
                .into();
            HttpResponse::Created().json(resp)
        }

        #[openapi(
            tag = "Mentors",
            operation_id = "mentor_login",
            request_body(
                description = "",
                content = LoginRequest
            ),
            responses(
                (status = 200, description = "", body = MentorAuthResponse),
                (status = 401, description = "", body = ApiError),
                (status = 400, description = "", body = ApiError),
            ),
        )]
        #[post("/login")]
        ///
        ///
        ///
        login(
            mentor_service: Data<MentorServiceDependency>,
            Json(body): Json<LoginRequest>,
        ) -> Json<MentorAuthResponse> {
            validate(&body)?;
            let res = mentor_service
                .login(body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Mentors",
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
        #[get("")]
        ///
        ///
        ///
        get_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
        ) -> Json<Mentor> {
            let mentor: Mentor = entity
                .into_inner()
                .try_into()?;
            let res = mentor_service
                .get_by_id(mentor.id)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Mentors",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("")]
        ///
        ///
        ///
        update_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Json(body): Json<MentorUpdate>,
        ) -> Json<Mentor> {
            let mentor: Mentor = entity
                .into_inner()
                .try_into()?;
            validate(&body)?;
            let res = mentor_service
                .update_by_id(mentor.id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Mentors",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/password")]
        ///
        ///
        ///
        change_password_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Json(body): Json<PasswordChangeRequest>,
        ) -> Json<MentorAuthResponse> {
            let mentor: Mentor = entity
                .into_inner()
                .try_into()?;
            validate(&body)?;
            let res = mentor_service
                .change_password_by_id(mentor.id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Mentors",
            operation_id = "delete_current_mentor",
            security(
                ("mentor" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("")]
        ///
        ///
        ///
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

        #[openapi(
            tag = "Mentors",
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
        #[get("")]
        ///
        ///
        ///
        get_by_id(
            mentor_service: Data<MentorServiceDependency>,
            Path(mentor_id): Path<Ulid>,
        ) -> Json<Mentor> {
            let res = mentor_service
                .get_by_id(mentor_id)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Mentors",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("")]
        ///
        ///
        ///
        update_by_id(
            mentor_service: Data<MentorServiceDependency>,
            Path(mentor_id): Path<Ulid>,
            Json(body): Json<MentorUpdate>,
        ) -> Json<Mentor> {
            validate(&body)?;
            let res = mentor_service
                .update_by_id(mentor_id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Mentors",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/password")]
        ///
        ///
        ///
        change_password_by_id(
            mentor_service: Data<MentorServiceDependency>,
            Path(mentor_id): Path<Ulid>,
            Json(body): Json<PasswordChangeRequest>,
        ) -> Json<MentorAuthResponse> {
            validate(&body)?;
            let res = mentor_service
                .change_password_by_id(mentor_id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Mentors",
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
        #[delete("")]
        ///
        ///
        ///
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
