use actix_multipart::form::MultipartForm;
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

use super::{ProfileHandler, ProfileHandlerHelper, ProfileHandlerResult};
use crate::{
    common::{ApiError, ValidationError},
    profile::UploadForm,
};

handler_implementation! {
    ProfileHandler as Implemented {
        ///
        ///
        ///
        #[openapi(
            operation_id = "upsert_current_profile",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            request_body(
                description = "",
                content = UpsertProfile
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
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
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
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
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
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
            security(
                ("organizator" = []),
            ),
            params(
                ("profile_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = UpsertProfile
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
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
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                ("profile_id" = Ulid, description = "")
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
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
            security(
                ("organizator" = []),
            ),
            params(
                ("profile_id" = Ulid, description = "")
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
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

        ///
        ///
        ///
        #[openapi(
            operation_id = "upsert_current_profile_image",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            request_body(
                description = "",
                content = UploadForm,
                content_type = "multipart/form-data",
            ),
            responses(
                (status = 200, description = ""),
                (status = 415, description = "", body = ApiError),
                (status = 413, description = "", body = ApiError),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/me/image")]
        upsert_image_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            MultipartForm(form): MultipartForm<UploadForm>,
        ) -> HttpResponse {
            profile_service
               .upsert_image_by_id(user.id, form.file)
               .await?;

            HttpResponse::Ok().finish()
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_current_profile_image",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("/me/image")]
        get_image_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse {
            let res = profile_service
                .get_image_by_id(user.id)
                .await?;
            HttpResponse::Ok()
               .content_type(res.content_type.clone())
               .body(res.data)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_current_profile_image",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/me/image")]
        delete_image_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse {
            profile_service
                .delete_image_by_id(user.id)
                .await?;
            HttpResponse::NoContent().finish()
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "upsert_profile_image_by_id",
            security(
                ("organizator" = []),
            ),
            params(
                ("profile_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = UploadForm,
                content_type = "multipart/form-data",
            ),
            responses(
                (status = 200, description = ""),
                (status = 415, description = "", body = ApiError),
                (status = 413, description = "", body = ApiError),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/{profile_id}/image")]
        upsert_image_by_id(
            profile_service: Data<ProfileServiceDependency>,
            Path(profile_id): Path<Ulid>,
            MultipartForm(form): MultipartForm<UploadForm>,
        ) -> HttpResponse {
            profile_service
               .upsert_image_by_id(profile_id, form.file)
               .await?;

            HttpResponse::Ok().finish()
        }


        ///
        ///
        ///
        #[openapi(
            operation_id = "get_profile_image_by_id",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                ("profile_id" = Ulid, description = "")
            ),
            responses(
                (status = 200, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{profile_id}/image")]
        get_image_by_id(
            profile_service: Data<ProfileServiceDependency>,
            Path(profile_id): Path<Ulid>,
        ) -> HttpResponse {
            let res = profile_service
                .get_image_by_id(profile_id)
                .await?;
            HttpResponse::Ok()
              .content_type(res.content_type.clone())
              .body(res.data)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_profile_image_by_id",
            security(
                ("organizator" = []),
            ),
            params(
                ("profile_id" = Ulid, description = "")
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/{profile_id}/image")]
        delete_image_by_id(
            profile_service: Data<ProfileServiceDependency>,
            Path(profile_id): Path<Ulid>,
        ) -> HttpResponse {
            profile_service
                .delete_image_by_id(profile_id)
                .await?;
            HttpResponse::NoContent().finish()
        }
    }
}
