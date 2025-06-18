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
use ulid::Ulid;
use utils::{
    LGTM,
    auth::{PasswordHasher, jwt},
};

use super::{
    DEFAULT_ADMIN_ID, USERS_BY_ROLE_COUNT_METRIC_NAME, UserService,
    UserServiceResult,
};
use crate::common::{ServiceError, wrapper::JwtSecret};

#[implementation(result = UserServiceResult)]
pub mod service {
    struct UserServiceImpl {
        user_repository: UserRepositoryDependency,
        secret: JwtSecret,
        password_hasher: PasswordHasher,
    }

    impl UserService for UserServiceImpl {
        async fn register(&self, new: CreateUser) -> (User, String) {
            if self
                .user_repository
                .exists_by_username(&new.username)
                .await?
            {
                Err(ServiceError::AlreadyExists(
                    "User with provided username".into(),
                ))?
            }

            let user = self
                .user_repository
                .save(CreateUserEntity {
                    email: new.email,
                    password_hash: self.password_hasher.hash(&new.password)?,
                    username: new.username,
                    role: new.role.into(),
                })
                .await?;

            let token = generate_jwt(&user, &*self.secret);

            (user.into(), token)
        }

        async fn login(
            &self,
            LoginRequest {
                email,
                password,
            }: LoginRequest,
        ) -> (User, String) {
            let user =
                self.user_repository.find_by_email(&email).await?.ok_or(
                    ServiceError::NotFound(
                        "User with provided username".into(),
                    ),
                )?;

            self.password_hasher
                .verify(&password, &user.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let token = generate_jwt(&user, &self.secret);

            (user.into(), token)
        }

        async fn find_by_id(&self, id: Ulid) -> Option<User> {
            self.user_repository
                .find_by_id(id.into())
                .await?
                .map(User::from)
        }

        async fn get_by_id(&self, id: Ulid) -> User {
            self.find_by_id(id)
                .await?
                .ok_or(ServiceError::NotFound("User with provided id".into()))?
        }

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
                if self.user_repository.exists_by_username(username).await? {
                    Err(ServiceError::AlreadyExists(
                        "User with provided username".into(),
                    ))?
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
                    },
                )
                .await?
                .expect("Got unchecked self ID")
                .into()
        }

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
            let user =
                self.user_repository.find_by_id(id.into()).await?.ok_or(
                    ServiceError::NotFound("User with provided id".into()),
                )?;

            PasswordHasher::new()
                .verify(&current_password, &user.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let user = self
                .user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        password_hash: Some(new_password),
                        ..Default::default()
                    },
                )
                .await?
                .expect("Got unchecked self ID");

            let token = generate_jwt(&user, &self.secret);

            (user.into(), token)
        }

        async fn delete_by_id(&self, id: Ulid, check_user: bool) -> () {
            if id.to_string() == DEFAULT_ADMIN_ID {
                Err(ServiceError::Forbidden(
                    "Unable to delete specified user".to_string(),
                ))?
            }
            if check_user {
                self.get_by_id(id).await?;
            }
            self.user_repository.delete_by_id(id.into()).await?;
        }

        async fn init_metrics(&self) {
            describe_gauge!(
                USERS_BY_ROLE_COUNT_METRIC_NAME,
                "The number of users by role"
            );

            let user_repository = self.user_repository.clone();
            tokio::spawn(async move {
                loop {
                    if let Ok(users_by_role) =
                        user_repository.count_by_role().await
                    {
                        users_by_role.iter().for_each(|(role, count)| {
                            gauge!(USERS_BY_ROLE_COUNT_METRIC_NAME, "role" => role.to_string()).set(*count);
                        });
                    }

                    tokio::time::sleep(LGTM::METRIC_SCRAPE_INTERVAL).await;
                }
            });
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn generate_jwt(user: &UserEntity, secret: &str) -> String {
    jwt::new(&user.role.to_string(), user.id.clone().into(), secret)
}
