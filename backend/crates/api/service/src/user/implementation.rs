use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    user::{CreateUser, User, UserUpdate},
};
use macros::implementation;
use repository::user::{
    CreateUser as CreateUserEntity, User as UserEntity,
    UserRepositoryDependency, UserUpdate as UserEntityUpdate,
};
use ulid::Ulid;
use utils::auth::{jwt, password_hashing::PasswordHasher};

use crate::common::ServiceError;

const DEFAULT_ADMIN_ID: &str = "00000000002YJ5PTSW7R6DCSTQ";

implementation! {
    UserService {
        user_repository: UserRepositoryDependency,
        secret: String,
        password_hasher: PasswordHasher<'static>
    } as Implemented {
        register(
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
                        username: new.username.clone(),
                        role: new.role.into(),
                    }
                ).await?;

            let token = generate_jwt(&user, &self.secret);

            (user.into(), token)
        }

        login(
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

        find_by_id(
            &self,
            id: Ulid,
        ) -> Option<User> {
            self.user_repository
                .find_by_id(id.into())
                .await?
                .map(User::from)
        }

        get_by_id(
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

        update_by_id(
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
            self.get_by_id(id).await?;
            if update.username.is_some()
                && self.user_repository
                    .exists_by_username(update.username.as_ref().unwrap())
                    .await? {
                Err(ServiceError::AlreadyExists("User with provided username".into()))?
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
                .unwrap()
                .into()
        }

        change_password_by_id(
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
                    ServiceError::NotFound("User with provided username".into())
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
                .unwrap();

            let token = generate_jwt(&user, &self.secret);

            (user.into(), token)
        }

        delete_by_id(
            &self,
            id: Ulid,
        ) -> () {
            if id.to_string() == DEFAULT_ADMIN_ID {
                Err(ServiceError::Forbidden(
                    "Unable to delete specified user".to_string(),
                ))?
            }
            self.get_by_id(id).await?;
            self.user_repository
                .delete_by_id(id.into())
                .await?;
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn generate_jwt(user: &UserEntity, secret: &str) -> String {
    jwt::new(&user.role.to_string(), user.id.clone().into(), secret)
}
