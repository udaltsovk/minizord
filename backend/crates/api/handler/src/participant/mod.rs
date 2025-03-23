use crate::common::middleware::auth::{
    AuthEntity, any_auth_middleware, organizator_auth_middleware,
    participant_auth_middleware,
};
use actix_web::{
    HttpResponse,
    middleware::from_fn,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    participant::{CreateParticipant, Participant, ParticipantUpdate},
};
use macros::{handler, response};
use service::participant::ParticipantServiceDependency;
use ulid::Ulid;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub mod implementation;

handler! {
    Participant with impl(ImplementedParticipantHandler) {
        #routes(participant_service: ParticipantServiceDependency) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(participant_service))
                    .service(scope("/participants")
                        .service(scope("")
                            .wrap(from_fn(organizator_auth_middleware))
                            .service(Self::register())
                        )
                        .service(Self::login())
                        .service(scope("/me")
                            .wrap(from_fn(participant_auth_middleware))
                            .service(Self::get_current())
                            .service(Self::update_current())
                            .service(Self::change_password_current())
                            .service(Self::delete_current())
                        )
                        .service(scope("/{participant_id}")
                            .service(scope("")
                                .wrap(from_fn(any_auth_middleware))
                                .service(Self::get_by_id())
                            )
                            .service(scope("")
                                .wrap(from_fn(organizator_auth_middleware))
                                .service(Self::update_by_id())
                                .service(Self::change_password_by_id())
                                .service(Self::delete_by_id())
                            )
                        )
                    );
            }
        }

        register(
            participant_service: Data<ParticipantServiceDependency>,
            body: Json<CreateParticipant>
        ) -> HttpResponse;

        login(
            participant_service: Data<ParticipantServiceDependency>,
            body: Json<LoginRequest>
        ) -> Json<ParticipantAuthResponse>;

        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Participant>;

        update_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
            body: Json<ParticipantUpdate>
        ) -> Json<Participant>;

        change_password_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
            body: Json<PasswordChangeRequest>
        ) -> Json<ParticipantAuthResponse>;

        delete_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
        ) -> HttpResponse;

        get_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            participant_id: Path<Ulid>
        ) -> Json<Participant>;

        update_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            participant_id: Path<Ulid>,
            body: Json<ParticipantUpdate>
        ) -> Json<Participant>;

        change_password_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            participant_id: Path<Ulid>,
            body: Json<PasswordChangeRequest>
        ) -> Json<ParticipantAuthResponse>;

        delete_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            participant_id: Path<Ulid>
        ) -> HttpResponse;
    }
}

response! {
    ///
    ParticipantAuth {
        ///
        token: String,
        ///
        participant: Participant
    }
}

impl From<(Participant, String)> for ParticipantAuthResponse {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from((participant, token): (Participant, String)) -> Self {
        Self { token, participant }
    }
}
