use actix_web::{
    HttpResponse,
    middleware::from_fn,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    profile::{CreateProfile, Profile, ProfileUpdate},
    user::User,
};
use macros::handler;
use service::profile::ProfileServiceDependency;
use ulid::Ulid;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::middleware::auth::{
    auth_middleware, organizator_auth_middleware,
};

pub mod implementation;

handler! {
    Profile with impl(ImplementedProfileHandler) {
        #routes(profile_service: ProfileServiceDependency) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(profile_service))
                    .service(scope("/profiles")
                        .wrap(from_fn(auth_middleware))
                        .service(Self::create())
                        .service(Self::get_current())
                        .service(Self::update_current())
                        .service(Self::delete_current())
                        .service(Self::get_by_id())
                        .service(scope("")
                            .wrap(from_fn(organizator_auth_middleware))
                            .service(Self::update_by_id())
                            .service(Self::delete_by_id())
                    ));
            }
        }

        create(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            body: Validated<Json<CreateProfile>>,
        ) -> HttpResponse;

        get_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> Json<Profile>;

        update_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
            body: Validated<Json<ProfileUpdate>>,
        ) -> Json<Profile>;

        delete_current(
            profile_service: Data<ProfileServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        get_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> Json<Profile>;

        update_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
            body: Validated<Json<ProfileUpdate>>,
        ) -> Json<Profile>;

        delete_by_id(
            profile_service: Data<ProfileServiceDependency>,
            profile_id: Path<Ulid>,
        ) -> HttpResponse;
    }
}
