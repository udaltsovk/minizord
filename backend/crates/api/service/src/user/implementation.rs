use std::time::Duration;

use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    user::{CreateUser, User, UserUpdate},
};
use entity::user::{
    CreateUser as CreateUserEntity, User as UserEntity,
    UserUpdate as UserEntityUpdate,
};
use macros::implementation;
use metrics::{describe_gauge, gauge};
use repository::user::UserRepositoryDependency;
use tracing::instrument;
use ulid::Ulid;
use utils::auth::{PasswordHasher, jwt};

use super::{
    USERS_BY_ROLE_METRIC_NAME, USERS_REGISTERED_METRIC_NAME, UserService,
    UserServiceResult,
};
use crate::common::ServiceError;

const DEFAULT_ADMIN_ID: &str = "0000000000000000000000000A";

implementation! {
    UserService {
        user_repository: UserRepositoryDependency,
        secret: String,
        password_hasher: PasswordHasher<'static>
    } as UserServiceImpl {
        #[instrument(skip_all, name = "UserService::register")]
        async fn register(
            &self,
            new: CreateUser,
        ) -> (User, String) {
            if self.user_repository.exists_by_username(&new.username).await? {
                Err(ServiceError::AlreadyExists("User with provided username".into()))?
            }

            let user = self.user_repository
                .save(
                    CreateUserEntity {
                        email: new.email,
                        password_hash: self.password_hasher.hash(&new.password)?,
                        username: new.username,
                        role: new.role.into(),
                    }
                ).await?;

            let token = generate_jwt(&user, &self.secret);

            (user.into(), token)
        }

        #[instrument(skip_all, name = "UserService::login")]
        async fn login(
            &self,
            LoginRequest {
                email,
                password
            }: LoginRequest,
        ) -> (User, String) {
            let user = self.user_repository
                .find_by_email(&email)
                .await?
                .ok_or(
                    ServiceError::NotFound("User with provided username".into())
                )?;

            self.password_hasher
                .verify(&password, &user.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let token = generate_jwt(&user, &self.secret);

            (user.into(), token)
        }

        #[instrument(skip_all, name = "UserService::find_by_id")]
        async fn find_by_id(
            &self,
            id: Ulid,
        ) -> Option<User> {
            self.user_repository
                .find_by_id(id.into())
                .await?
                .map(User::from)
        }

        #[instrument(skip_all, name = "UserService::get_by_id")]
        async fn get_by_id(
            &self,
            id: Ulid,
        ) -> User {
            self
                .find_by_id(id)
                .await?
                .ok_or(
                    ServiceError::NotFound("User with provided id".into())
                )?
        }

        #[instrument(skip_all, name = "UserService::update_by_id")]
        async fn update_by_id(
            &self,
            id: Ulid,
            update: UserUpdate,
            is_self: bool,
        ) -> User {
            if !is_self && id.to_string() == DEFAULT_ADMIN_ID {
                Err(ServiceError::Forbidden(
                    "Unable to update specified user".to_string(),
                ))?
            }
            if !is_self {
                self.get_by_id(id).await?;
            }
            if let Some(username) = update.username.as_ref() {
                if self.user_repository
                    .exists_by_username(username)
                    .await?
                {
                    Err(ServiceError::AlreadyExists("User with provided username".into()))?
                }
            }

            self.user_repository
                .update_by_id(
                    id.into(),
                        UserEntityUpdate {
                        email: update.email,
                        password_hash: None,
                        username: update.username,
                        role: None,
                        profile: None,
                    }
                )
                .await?
                .expect("Got unchecked self ID")
                .into()
        }

        #[instrument(skip_all, name = "UserService::change_password_by_id")]
        async fn change_password_by_id(
            &self,
            id: Ulid,
            PasswordChangeRequest {
                current_password,
                new_password,
                ..
            }: PasswordChangeRequest,
            is_self: bool,
        ) -> (User, String) {

            if !is_self && id.to_string() == DEFAULT_ADMIN_ID {
                Err(ServiceError::Forbidden(
                    "Unable to update specified user's password".to_string(),
                ))?
            }
            let user = self.user_repository
                .find_by_id(id.into())
                .await?
                .ok_or(
                    ServiceError::NotFound("User with provided id".into())
                )?;

            PasswordHasher::new()
                .verify(&current_password, &user.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let user = self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        password_hash: Some(new_password),
                        ..Default::default()
                    }
                )
                .await?
                .expect("Got unchecked self ID");

            let token = generate_jwt(&user, &self.secret);

            (user.into(), token)
        }

        #[instrument(skip_all, name = "UserService::delete_by_id")]
        async fn delete_by_id(
            &self,
            id: Ulid,
            check_user: bool,
        ) -> () {
            if id.to_string() == DEFAULT_ADMIN_ID {
                Err(ServiceError::Forbidden(
                    "Unable to delete specified user".to_string(),
                ))?
            }
            if check_user {
                self.get_by_id(id).await?;
            }
            self.user_repository
                .delete_by_id(id.into())
                .await?;
        }

        #[instrument(skip_all, name = "UserService::init_metrics")]
        async fn init_metrics(&self) {
            describe_gauge!(USERS_REGISTERED_METRIC_NAME, "The number of currently registered users");
            describe_gauge!(USERS_BY_ROLE_METRIC_NAME, "The number of users by role");

            let user_repository = self.user_repository.clone();
            tokio::spawn(async move {
                loop {
                    if let Ok(registered_users) = user_repository
                        .count_registered()
                        .await
                    {
                        gauge!(USERS_REGISTERED_METRIC_NAME).set(registered_users);
                    }
                    if let Ok(users_by_role) = user_repository
                        .count_by_role()
                        .await
                    {
                        users_by_role.into_iter().for_each(|(role, count)| {
                            gauge!(USERS_BY_ROLE_METRIC_NAME, "role" => role).set(count);
                        });
                    }

                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            });
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn generate_jwt(user: &UserEntity, secret: &str) -> String {
    jwt::new(&user.role.to_string(), user.id.clone().into(), secret)
}
