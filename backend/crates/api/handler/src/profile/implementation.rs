use actix_web::{
    HttpResponse, delete, get, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    profile::{Profile, UpsertProfile},
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
            operation_id = "upsert_current_profile",
            request_body(
                description = "",
                content = UpsertProfile
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 409, description = "", body = HandlerError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[put("/me")]
        upsert_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            Validated(Json(body)): Validated<Json<UpsertProfile>>
        ) -> Json<Profile> {
            let resp: Profile = profile_service
                .upsert_by_id(user.id, body)
                .await?;
            Json(resp)
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
            operation_id = "upsert_profile_by_id",
            params(
                ("profile_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = UpsertProfile
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
        #[put("/{profile_id}")]
        upsert_by_id(
            profile_service: Data<ProfileServiceDependency>,
            Path(profile_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<UpsertProfile>>,
        ) -> Json<Profile> {
            let res = profile_service
                .upsert_by_id(profile_id, body)
                .await?;
            Json(res)
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
