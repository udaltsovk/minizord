use actix_web::{
    HttpResponse,
    middleware::from_fn,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    organizator::{CreateOrganizator, Organizator, OrganizatorUpdate},
};
use macros::{handler, response};
use service::organizator::OrganizatorServiceDependency;
use ulid::Ulid;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::middleware::auth::{
    AuthEntity, any_auth_middleware, organizator_auth_middleware,
};

pub mod implementation;

handler! {
    Organizator with impl(ImplementedOrganizatorHandler) {
        #routes(organizator_service: OrganizatorServiceDependency) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(organizator_service))
                    .service(scope("/organizators")
                        .service(Self::register())
                        .service(Self::login())
                        .service(scope("/me")
                            .wrap(from_fn(organizator_auth_middleware))
                            .service(Self::get_current())
                            .service(Self::update_current())
                            .service(Self::change_password_current())
                            .service(Self::delete_current())
                        )
                        .service(scope("/{organizator_id}")
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
            organizator_service: Data<OrganizatorServiceDependency>,
            body: Json<CreateOrganizator>
        ) -> HttpResponse;

        login(
            organizator_service: Data<OrganizatorServiceDependency>,
            body: Json<LoginRequest>
        ) -> Json<OrganizatorAuthResponse>;

        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Organizator>;

        update_current(
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
            body: Json<OrganizatorUpdate>
        ) -> Json<Organizator>;

        change_password_current(
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
            body: Json<PasswordChangeRequest>
        ) -> Json<OrganizatorAuthResponse>;

        delete_current(
            organizator_service: Data<OrganizatorServiceDependency>,
            entity: ReqData<AuthEntity>,
        ) -> HttpResponse;

        get_by_id(
            organizator_service: Data<OrganizatorServiceDependency>,
            organizator_id: Path<Ulid>
        ) -> Json<Organizator>;

        update_by_id(
            organizator_service: Data<OrganizatorServiceDependency>,
            organizator_id: Path<Ulid>,
            body: Json<OrganizatorUpdate>
        ) -> Json<Organizator>;

        change_password_by_id(
            organizator_service: Data<OrganizatorServiceDependency>,
            organizator_id: Path<Ulid>,
            body: Json<PasswordChangeRequest>
        ) -> Json<OrganizatorAuthResponse>;

        delete_by_id(
            organizator_service: Data<OrganizatorServiceDependency>,
            organizator_id: Path<Ulid>
        ) -> HttpResponse;
    }
}

response! {
    ///
    OrganizatorAuth {
        ///
        token: String,
        ///
        organizator: Organizator
    }
}

impl From<(Organizator, String)> for OrganizatorAuthResponse {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from((organizator, token): (Organizator, String)) -> Self {
        Self {
            token,
            organizator,
        }
    }
}
