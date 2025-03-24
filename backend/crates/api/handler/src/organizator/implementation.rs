use actix_web::{
    HttpResponse, delete, get, patch, post, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    organizator::{CreateOrganizator, Organizator, OrganizatorUpdate},
};
use macros::handler_implementation;
use service::organizator::OrganizatorServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

use super::OrganizatorAuthResponse;
use crate::common::{ApiError, ValidationError, middleware::auth::AuthEntity};

handler_implementation! {
    OrganizatorHandler as Implemented {
        ///
        ///
        ///
        #[openapi(
            operation_id = "register_organizator",
            request_body(
                description = "",
                content = CreateOrganizator
            ),
            responses(
                (status = 201, description = "", body = OrganizatorAuthResponse),
                (status = 409, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/register")]
        register(
            organizator_service: Data<OrganizatorServiceDependency>,
            Validated(Json(body)): Validated<Json<CreateOrganizator>>
        ) -> HttpResponse {
            let resp: OrganizatorAuthResponse = organizator_service
                .register(body)
                .await?
                .into();
            HttpResponse::Created().json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "organizator_login",
            request_body(
                description = "",
                content = LoginRequest
            ),
            responses(
                (status = 200, description = "", body = OrganizatorAuthResponse),
                (status = 401, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/login")]
        login(
            organizator_service: Data<OrganizatorServiceDependency>,
            Validated(Json(body)): Validated<Json<LoginRequest>>,
        ) -> Json<OrganizatorAuthResponse> {
            let res = organizator_service
                .login(body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
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
        #[get("/me")]
        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Organizator> {
            let organizator: Organizator = entity
                .into_inner()
                .try_into()?;
            Json(organizator)
        }

        ///
        ///
        ///
        #[openapi(
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
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/me")]
        update_current(
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Validated(Json(body)): Validated<Json<OrganizatorUpdate>>,
        ) -> Json<Organizator> {
            let organizator: Organizator = entity
                .into_inner()
                .try_into()?;

            if body.username.as_ref().unwrap_or(&organizator.username) == &organizator.username {
                return Ok(Json(organizator));
            }

            let res = organizator_service
                .update_by_id(organizator.id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
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
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/me/password")]
        change_password_current(
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<OrganizatorAuthResponse> {
            let organizator: Organizator = entity
                .into_inner()
                .try_into()?;
            let res = organizator_service
                .change_password_by_id(organizator.id, body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_current_organizator",
            security(
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

        ///
        ///
        ///
        #[openapi(
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
        #[get("/{organizator_id}")]
        get_by_id(
            organizator_service: Data<OrganizatorServiceDependency>,
            Path(organizator_id): Path<Ulid>,
        ) -> Json<Organizator> {
            let res = organizator_service
                .get_by_id(organizator_id)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
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
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/{organizator_id}")]
        update_by_id(
            organizator_service: Data<OrganizatorServiceDependency>,
            Path(organizator_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<OrganizatorUpdate>>,
        ) -> Json<Organizator> {
            let res = organizator_service
                .update_by_id(organizator_id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
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
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/{organizator_id}/password")]
        change_password_by_id(
            organizator_service: Data<OrganizatorServiceDependency>,
            Path(organizator_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<OrganizatorAuthResponse> {
            let res = organizator_service
                .change_password_by_id(organizator_id, body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
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
        #[delete("/{organizator_id}")]
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
