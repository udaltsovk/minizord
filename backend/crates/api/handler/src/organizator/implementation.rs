use super::OrganizatorAuthResponse;
use crate::common::{ApiError, middleware::auth::AuthEntity, validate};
use actix_web::{
    HttpResponse, delete, get, patch, post, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    organizator::{CreateOrganizator, Organizator, OrganizatorUpdate},
};
use macros::handler_implementation;
use service::organizator::OrganizatorServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

handler_implementation! {
    OrganizatorHandler as Implemented {
        #[openapi(
            tag = "Organizators",
            operation_id = "register_organizator",
            request_body(
                description = "",
                content = CreateOrganizator
            ),
            responses(
                (status = 201, description = "", body = OrganizatorAuthResponse),
                (status = 409, description = "", body = ApiError),
                (status = 400, description = "", body = ApiError),
            ),
        )]
        #[post("/register")]
        ///
        ///
        ///
        register(
            organizator_service: Data<OrganizatorServiceDependency>,
            Json(body): Json<CreateOrganizator>
        ) -> HttpResponse {
            validate(&body)?;
            let resp: OrganizatorAuthResponse = organizator_service
                .register(body)
                .await?
                .into();
            HttpResponse::Created().json(resp)
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "organizator_login",
            request_body(
                description = "",
                content = LoginRequest
            ),
            responses(
                (status = 200, description = "", body = OrganizatorAuthResponse),
                (status = 401, description = "", body = ApiError),
                (status = 400, description = "", body = ApiError),
            ),
        )]
        #[post("/login")]
        ///
        ///
        ///
        login(
            organizator_service: Data<OrganizatorServiceDependency>,
            Json(body): Json<LoginRequest>,
        ) -> Json<OrganizatorAuthResponse> {
            validate(&body)?;
            let res = organizator_service
                .login(body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "get_current_organizator",
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Organizator),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("")]
        ///
        ///
        ///
        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Organizator> {
            let organizator: Organizator = entity
                .into_inner()
                .try_into()?;
            Json(organizator.into())
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "update_current_organizator",
            request_body(
                description = "",
                content = OrganizatorUpdate
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Organizator),
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
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Json(body): Json<OrganizatorUpdate>,
        ) -> Json<Organizator> {
            let organizator: Organizator = entity
                .into_inner()
                .try_into()?;
            validate(&body)?;

            if body.username.as_ref().unwrap_or(&organizator.username) == &organizator.username {
                return Ok(Json(organizator.into()));
            }

            let res = organizator_service
                .update_by_id(organizator.id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "change_current_organizator_password",
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = OrganizatorAuthResponse),
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
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Json(body): Json<PasswordChangeRequest>,
        ) -> Json<OrganizatorAuthResponse> {
            let organizator: Organizator = entity
                .into_inner()
                .try_into()?;
            validate(&body)?;
            let res = organizator_service
                .change_password_by_id(organizator.id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "delete_current_organizator",
            security(
                ("organizator" = []),
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
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
        ) -> HttpResponse {
            let organizator: Organizator = entity
                .into_inner()
                .try_into()?;
            organizator_service
                .delete_by_id(organizator.id)
                .await?;
            HttpResponse::NoContent().into()
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "get_organizator_by_id",
            params(
                ("organizator_id" = Ulid, description = "")
            ),
            security(
                ("organizator" = []),
                ("mentor" = []),
                ("participant" = []),
            ),
            responses(
                (status = 200, description = "", body = Organizator),
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
            organizator_service: Data<OrganizatorServiceDependency>,
            Path(organizator_id): Path<Ulid>,
        ) -> Json<Organizator> {
            let res = organizator_service
                .get_by_id(organizator_id)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "update_organizator_by_id",
            params(
                ("organizator_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = OrganizatorUpdate
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Organizator),
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
            organizator_service: Data<OrganizatorServiceDependency>,
            Path(organizator_id): Path<Ulid>,
            Json(body): Json<OrganizatorUpdate>,
        ) -> Json<Organizator> {
            validate(&body)?;
            let res = organizator_service
                .update_by_id(organizator_id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "change_organizator_password_by_id",
            params(
                ("organizator_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = OrganizatorAuthResponse),
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
            organizator_service: Data<OrganizatorServiceDependency>,
            Path(organizator_id): Path<Ulid>,
            Json(body): Json<PasswordChangeRequest>,
        ) -> Json<OrganizatorAuthResponse> {
            validate(&body)?;
            let res = organizator_service
                .change_password_by_id(organizator_id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Organizators",
            operation_id = "delete_organizator_by_id",
            params(
                ("organizator_id" = Ulid, description = "")
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
            organizator_service: Data<OrganizatorServiceDependency>,
            Path(organizator_id): Path<Ulid>,
        ) -> HttpResponse {
            organizator_service
                .delete_by_id(organizator_id)
                .await?;
            HttpResponse::NoContent().into()
        }
    }
}
