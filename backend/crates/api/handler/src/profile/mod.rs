use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{
    HttpResponse,
    middleware::from_fn,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    profile::{Profile, UpsertProfile},
    user::{User, UserRole},
};
use macros::handler;
use service::{
    profile::ProfileServiceDependency,
    profile_image::ProfileImageServiceDependency,
};
use ulid::Ulid;
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::{
    HandlerError, guard::UserRoleGuard, middleware::user_extractor_middleware,
};

pub mod implementation;

handler! {
    Profile
        Err: HandlerError,
        Impl: ImplementedProfileHandler
    {
        fn routes(
            profile_service: ProfileServiceDependency,
            profile_image_service: ProfileImageServiceDependency,
        ) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(profile_service))
                    .app_data(Data::new(profile_image_service))
                    .service(scope("/profiles")
                        .wrap(from_fn(user_extractor_middleware))
                        .service(Self::get_current_profile())
                        .service(Self::upsert_current_profile())
                        .service(Self::delete_current_profile())
                        .service(Self::get_current_profile_image())
                        .service(Self::upsert_current_profile_image())
                        .service(Self::delete_current_profile_image())
                        .service(Self::get_profile_by_id())
                        .service(Self::get_profile_image_by_id())
                        .service(scope("")
                            .guard(UserRoleGuard::new(&[UserRole::Organizer]))
                            .service(Self::delete_profile_by_id())
                            .service(Self::delete_profile_image_by_id())
                        )
                    );
            }
        }

        async fn get_current_profile(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> Json<Profile>;

        async fn upsert_current_profile(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            body: Validated<Json<UpsertProfile>>,
        ) -> Json<Profile>;

        async fn delete_current_profile(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        async fn get_current_profile_image(
            profile_image_service: Data<ProfileImageServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        async fn upsert_current_profile_image(
            profile_image_service: Data<ProfileImageServiceDependency>,
            user: ReqData<User>,
            form: MultipartForm<UploadForm>,
        ) -> HttpResponse;

        async fn delete_current_profile_image(
            profile_image_service: Data<ProfileImageServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        async fn get_profile_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> Json<Profile>;

        async fn get_profile_image_by_id(
            profile_image_service: Data<ProfileImageServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> HttpResponse;

        async fn delete_profile_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> HttpResponse;

        async fn delete_profile_image_by_id(
            profile_image_service: Data<ProfileImageServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> HttpResponse;
    }
}

#[derive(MultipartForm, ToSchema, Debug)]
struct UploadForm {
    #[schema(
        value_type = String,
        format = Binary,
        content_media_type = "application/octet-stream",
    )]
    /// Изображение, которое будет загружено.
    /// Размер не должен превышать 5.7 МБ.
    /// Разрешённые MIME типы: `image/jpeg`, `image/pjpeg`, `image/png`, `image/webp`
    file: TempFile,
}
