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
use service::{
    profile::ProfileServiceDependency,
    profile_image::ProfileImageServiceDependency,
};
use tracing::instrument;
use ulid::Ulid;

use super::{
    ProfileHandler, ProfileHandlerHelper, ProfileHandlerResult, UploadForm,
};
use crate::common::{ApiError, ValidationError, openapi};

handler_implementation! {
    ProfileHandler as ProfileHandlerImpl {
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
                (status = 200, description = "", body = Profile),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("/me")]
        #[instrument(skip_all, name = "ProfileHandler::get_current_profile")]
        async fn get_current_profile(
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
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
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
        #[instrument(skip_all, name = "ProfileHandler::upsert_current_profile")]
        async fn upsert_current_profile(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            Validated(Json(body)): Validated<Json<UpsertProfile>>
        ) -> Json<Profile> {
            let resp: Profile = profile_service
                .upsert_by_id(user.id, body, None)
                .await?;
            Json(resp)
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
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/me")]
        #[instrument(skip_all, name = "ProfileHandler::delete_current_profile")]
        async fn delete_current_profile(
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
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("/me/image")]
        #[instrument(skip_all, name = "ProfileHandler::get")]
        async fn get_current_profile_image(
            profile_image_service: Data<ProfileImageServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse {
            let res = profile_image_service
                .get_by_id(user.id)
                .await?;
            HttpResponse::Ok()
               .content_type(res.content_type)
               .body(res.data)
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
        async fn upsert_current_profile_image(
            profile_image_service: Data<ProfileImageServiceDependency>,
            user: ReqData<User>,
            MultipartForm(form): MultipartForm<UploadForm>,
        ) -> HttpResponse {
            profile_image_service
               .upsert_by_id(user.id, form.file)
               .await?;

            HttpResponse::Ok().finish()
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
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/me/image")]
        async fn delete_current_profile_image(
            profile_image_service: Data<ProfileImageServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse {
            profile_image_service
                .delete_by_id(user.id)
                .await?;
            HttpResponse::NoContent().finish()
        }



        ///
        ///
        ///
        #[openapi(
            params(
                ("profile_id" = Ulid, description = "")
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = Profile),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{profile_id}")]
        async fn get_profile_by_id(
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
            params(
                ("profile_id" = Ulid, description = "")
            ),
            security(
                ("organizer" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/{profile_id}")]
        async fn delete_profile_by_id(
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
            params(
                ("profile_id" = Ulid, description = "")
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{profile_id}/image")]
        async fn get_profile_image_by_id(
            profile_image_service: Data<ProfileImageServiceDependency>,
            Path(profile_id): Path<Ulid>,
        ) -> HttpResponse {
            let res = profile_image_service
                .get_by_id(profile_id)
                .await?;
            HttpResponse::Ok()
              .content_type(res.content_type)
              .body(res.data)
        }

        ///
        ///
        ///
        #[openapi(
            params(
                ("profile_id" = Ulid, description = "")
            ),
            security(
                ("organizer" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/{profile_id}/image")]
        async fn delete_profile_image_by_id(
            profile_image_service: Data<ProfileImageServiceDependency>,
            Path(profile_id): Path<Ulid>,
        ) -> HttpResponse {
            profile_image_service
                .delete_by_id(profile_id)
                .await?;
            HttpResponse::NoContent().finish()
        }
    }
}
