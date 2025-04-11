use actix_web::{
    HttpResponse,
    middleware::from_fn,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    user::{CreateUser, User, UserRole, UserUpdate},
};
use macros::{handler, response};
use service::user::UserServiceDependency;
use ulid::Ulid;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::{
    HandlerError,
    middleware::{UserRoleFilterMiddleware, user_extractor_middleware},
};

pub mod implementation;

handler! {
    User
        Err: HandlerError,
        Impl: ImplementedUserHandler
    {
        #routes(user_service: UserServiceDependency) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(user_service))
                    .service(scope("/users")
                        .service(Self::login())
                        .service(scope("")
                            .wrap(from_fn(user_extractor_middleware))
                            .service(Self::get_current())
                            .service(Self::update_current())
                            .service(Self::change_password_current())
                            .service(Self::delete_current())
                            .service(Self::get_by_id())
                            .service(scope("")
                                .wrap(UserRoleFilterMiddleware::new(vec![UserRole::Organizator]))
                                .service(Self::register())
                                .service(Self::update_by_id())
                                .service(Self::change_password_by_id())
                                .service(Self::delete_by_id())
                            )
                        )
                    );
            }
        }

        register(
            user_service: Data<UserServiceDependency>,
            body: Validated<Json<CreateUser>>
        ) -> HttpResponse;

        login(
            user_service: Data<UserServiceDependency>,
            body: Validated<Json<LoginRequest>>
        ) -> Json<UserAuthResponse>;

        get_current(
            user: ReqData<User>,
        ) -> Json<User>;

        update_current(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            body: Validated<Json<UserUpdate>>
        ) -> Json<User>;

        change_password_current(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
            body: Validated<Json<PasswordChangeRequest>>
        ) -> Json<UserAuthResponse>;

        delete_current(
            user_service: Data<UserServiceDependency>,
            user: ReqData<User>,
        ) -> HttpResponse;

        get_by_id(
            user_service: Data<UserServiceDependency>,
            user_id: Path<Ulid>
        ) -> Json<User>;

        update_by_id(
            user_service: Data<UserServiceDependency>,
            user_id: Path<Ulid>,
            body: Validated<Json<UserUpdate>>
        ) -> Json<User>;

        change_password_by_id(
            user_service: Data<UserServiceDependency>,
            user_id: Path<Ulid>,
            body: Validated<Json<PasswordChangeRequest>>
        ) -> Json<UserAuthResponse>;

        delete_by_id(
            user_service: Data<UserServiceDependency>,
            user_id: Path<Ulid>
        ) -> HttpResponse;
    }
}

response! {
    ///
    UserAuth {
        ///
        token: String,
        ///
        user: User
    }
}

impl From<(User, String)> for UserAuthResponse {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from((user, token): (User, String)) -> Self {
        Self {
            token,
            user,
        }
    }
}
