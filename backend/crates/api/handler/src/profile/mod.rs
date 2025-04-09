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
    user::User,
};
use macros::handler;
use service::profile::ProfileServiceDependency;
use ulid::Ulid;
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::{
    HandlerError,
    middleware::auth::{auth_middleware, organizator_auth_middleware},
};

pub mod implementation;

handler! {
    Profile
        Err: HandlerError,
        Impl: ImplementedProfileHandler
    {
        #routes(profile_service: ProfileServiceDependency) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(profile_service))
                    .service(scope("/profiles")
                        .wrap(from_fn(auth_middleware))
                        .service(Self::upsert_current())
                        .service(Self::get_current())
                        .service(Self::delete_current())
                        .service(Self::upsert_image_current())
                        .service(Self::get_image_current())
                        .service(Self::delete_image_current())
                        .service(Self::get_by_id())
                        .service(Self::get_image_by_id())
                        .service(scope("")
                            .wrap(from_fn(organizator_auth_middleware))
                            .service(Self::upsert_by_id())
                            .service(Self::delete_by_id())
                            .service(Self::upsert_image_by_id())
                            .service(Self::delete_image_by_id())
                        )
                    );
            }
        }

        upsert_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            body: Validated<Json<UpsertProfile>>,
        ) -> Json<Profile>;

        get_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> Json<Profile>;

        delete_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        upsert_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
            body: Validated<Json<UpsertProfile>>,
        ) -> Json<Profile>;

        get_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> Json<Profile>;

        delete_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> HttpResponse;

        upsert_image_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            form: MultipartForm<UploadForm>,
        ) -> HttpResponse;

        get_image_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        delete_image_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        upsert_image_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
            form: MultipartForm<UploadForm>,
        ) -> HttpResponse;

        get_image_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> HttpResponse;

        delete_image_by_id(
            profile_service: Data<ProfileServiceDependency>,
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
