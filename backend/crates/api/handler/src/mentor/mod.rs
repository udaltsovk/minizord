use actix_web::{
    HttpResponse,
    middleware::from_fn,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    mentor::{CreateMentor, Mentor, MentorUpdate},
};
use macros::{handler, response};
use service::mentor::MentorServiceDependency;
use ulid::Ulid;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::middleware::auth::{
    AuthEntity, any_auth_middleware, mentor_auth_middleware,
    organizator_auth_middleware,
};

pub mod implementation;

handler! {
    Mentor with impl(ImplementedMentorHandler) {
        #routes(mentor_service: MentorServiceDependency) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(mentor_service))
                    .service(scope("/mentors")
                        .service(Self::login())
                        .service(scope("")
                            .wrap(from_fn(organizator_auth_middleware))
                            .service(Self::register())
                            .service(Self::update_by_id())
                            .service(Self::change_password_by_id())
                            .service(Self::delete_by_id())
                        )
                        .service(scope("")
                            .wrap(from_fn(mentor_auth_middleware))
                            .service(Self::get_current())
                            .service(Self::update_current())
                            .service(Self::change_password_current())
                            .service(Self::delete_current())
                        )
                        .service(scope("")
                            .wrap(from_fn(any_auth_middleware))
                            .service(Self::get_by_id())
                        )
                    );
            }
        }

        register(
            mentor_service: Data<MentorServiceDependency>,
            body: Validated<Json<CreateMentor>>
        ) -> HttpResponse;

        login(
            mentor_service: Data<MentorServiceDependency>,
            body: Validated<Json<LoginRequest>>
        ) -> Json<MentorAuthResponse>;

        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Mentor>;

        update_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
            body: Validated<Json<MentorUpdate>>
        ) -> Json<Mentor>;

        change_password_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
            body: Validated<Json<PasswordChangeRequest>>
        ) -> Json<MentorAuthResponse>;

        delete_current(
            mentor_service: Data<MentorServiceDependency>,
            entity: ReqData<AuthEntity>,
        ) -> HttpResponse;

        get_by_id(
            mentor_service: Data<MentorServiceDependency>,
            mentor_id: Path<Ulid>
        ) -> Json<Mentor>;

        update_by_id(
            mentor_service: Data<MentorServiceDependency>,
            mentor_id: Path<Ulid>,
            body: Validated<Json<MentorUpdate>>
        ) -> Json<Mentor>;

        change_password_by_id(
            mentor_service: Data<MentorServiceDependency>,
            mentor_id: Path<Ulid>,
            body: Validated<Json<PasswordChangeRequest>>
        ) -> Json<MentorAuthResponse>;

        delete_by_id(
            mentor_service: Data<MentorServiceDependency>,
            mentor_id: Path<Ulid>
        ) -> HttpResponse;
    }
}

response! {
    ///
    MentorAuth {
        ///
        token: String,
        ///
        mentor: Mentor
    }
}

impl From<(Mentor, String)> for MentorAuthResponse {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from((mentor, token): (Mentor, String)) -> Self {
        Self { token, mentor }
    }
}
