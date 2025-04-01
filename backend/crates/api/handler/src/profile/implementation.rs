use actix_web::{
    HttpResponse, delete, get, patch, post,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    profile::{CreateProfile, Profile, ProfileUpdate},
    user::User,
};
use macros::handler_implementation;
use service::profile::ProfileServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

use crate::common::{HandlerError, ValidationError};

handler_implementation! {
    ProfileHandler as Implemented {
        ///
        ///
        ///
        #[openapi(
            operation_id = "create_profile",
            request_body(
                description = "",
                content = CreateProfile
            ),
            responses(
                (status = 201, description = "", body = Profile),
                (status = 409, description = "", body = HandlerError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/me")]
        create(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            Validated(Json(body)): Validated<Json<CreateProfile>>
        ) -> HttpResponse {
            let resp: Profile = profile_service
                .create(user.id, body)
                .await?;
            HttpResponse::Created().json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_current_profile",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 403, description = "", body = HandlerError),
                (status = 401, description = "", body = HandlerError),
            ),

        )]
        #[get("/me")]
        get_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> Json<Profile> {
            let res = profile_service
                .get_by_id(user.id)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_current_profile",
            request_body(
                description = "",
                content = ProfileUpdate
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 409, description = "", body = HandlerError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = HandlerError),
                (status = 401, description = "", body = HandlerError),
            ),
        )]
        #[patch("/me")]
        update_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            Validated(Json(body)): Validated<Json<ProfileUpdate>>,
        ) -> Json<Profile> {
            let res = profile_service
                .update_by_id(user.id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_current_profile",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = HandlerError),
                (status = 401, description = "", body = HandlerError),
            ),
        )]
        #[delete("/me")]
        delete_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse {
            profile_service
                .delete_by_id(user.id)
                .await?;
            HttpResponse::NoContent().finish()
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_profile_by_id",
            params(
                ("profile_id" = Ulid, description = "")
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 404, description = "", body = HandlerError),
                (status = 403, description = "", body = HandlerError),
                (status = 401, description = "", body = HandlerError),
            ),
        )]
        #[get("/{profile_id}")]
        get_by_id(
            profile_service: Data<ProfileServiceDependency>,
            Path(profile_id): Path<Ulid>,
        ) -> Json<Profile> {
            let res = profile_service
                .get_by_id(profile_id)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_profile_by_id",
            params(
                ("profile_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = ProfileUpdate
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 409, description = "", body = HandlerError),
                (status = 404, description = "", body = HandlerError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = HandlerError),
                (status = 401, description = "", body = HandlerError),
            ),
        )]
        #[patch("/{profile_id}")]
        update_by_id(
            profile_service: Data<ProfileServiceDependency>,
            Path(profile_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<ProfileUpdate>>,
        ) -> Json<Profile> {
            let res = profile_service
                .update_by_id(profile_id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_profile_by_id",
            params(
                ("profile_id" = Ulid, description = "")
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = HandlerError),
                (status = 401, description = "", body = HandlerError),
                (status = 403, description = "", body = HandlerError),
            ),
        )]
        #[delete("/{profile_id}")]
        delete_by_id(
            profile_service: Data<ProfileServiceDependency>,
            Path(profile_id): Path<Ulid>,
        ) -> HttpResponse {
            profile_service
                .delete_by_id(profile_id)
                .await?;
            HttpResponse::NoContent().finish()
        }
    }
}
