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
use macros::handler;
use serde::Serialize;
use service::user::UserServiceDependency;
use ulid::Ulid;
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::{
    HandlerError, guard::UserRoleGuard, middleware::user_extractor_middleware,
};

pub mod implementation;

#[handler(error = HandlerError)]
pub trait UserHandler {
    fn routes(user_service: UserServiceDependency, cfg: &mut ServiceConfig) {
        cfg.app_data(Data::new(user_service)).service(
            scope("/users").service(Self::user_login()).service(
                scope("")
                    .wrap(from_fn(user_extractor_middleware))
                    .service(Self::get_current_user())
                    .service(Self::update_current_user())
                    .service(Self::change_current_user_password())
                    .service(Self::delete_current_user())
                    .service(Self::get_user_by_id())
                    .service(
                        scope("")
                            .guard(UserRoleGuard::new(&[UserRole::Organizer]))
                            .service(Self::register_user())
                            .service(Self::update_user_by_id())
                            .service(Self::change_user_password_by_id())
                            .service(Self::delete_user_by_id()),
                    ),
            ),
        );
    }

    async fn register_user(
        user_service: Data<UserServiceDependency>,
        body: Validated<Json<CreateUser>>,
    ) -> HttpResponse;

    async fn user_login(
        user_service: Data<UserServiceDependency>,
        body: Validated<Json<LoginRequest>>,
    ) -> Json<UserAuthResponse>;

    async fn get_current_user(user: ReqData<User>) -> Json<User>;

    async fn update_current_user(
        user_service: Data<UserServiceDependency>,
        user: ReqData<User>,
        body: Validated<Json<UserUpdate>>,
    ) -> Json<User>;

    async fn change_current_user_password(
        user_service: Data<UserServiceDependency>,
        user: ReqData<User>,
        body: Validated<Json<PasswordChangeRequest>>,
    ) -> Json<UserAuthResponse>;

    async fn delete_current_user(
        user_service: Data<UserServiceDependency>,
        user: ReqData<User>,
    ) -> HttpResponse;

    async fn get_user_by_id(
        user_service: Data<UserServiceDependency>,
        user: ReqData<User>,
        user_id: Path<Ulid>,
    ) -> Json<User>;

    async fn update_user_by_id(
        user_service: Data<UserServiceDependency>,
        user: ReqData<User>,
        user_id: Path<Ulid>,
        body: Validated<Json<UserUpdate>>,
    ) -> Json<User>;

    async fn change_user_password_by_id(
        user_service: Data<UserServiceDependency>,
        user: ReqData<User>,
        user_id: Path<Ulid>,
        body: Validated<Json<PasswordChangeRequest>>,
    ) -> Json<UserAuthResponse>;

    async fn delete_user_by_id(
        user_service: Data<UserServiceDependency>,
        user: ReqData<User>,
        user_id: Path<Ulid>,
    ) -> HttpResponse;
}

#[derive(Serialize, ToSchema, Debug, Clone)]
///
pub struct UserAuthResponse {
    ///
    pub token: String,
    ///
    pub user: User,
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
