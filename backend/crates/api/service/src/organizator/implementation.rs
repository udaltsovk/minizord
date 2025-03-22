use crate::common::ServiceError;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    organizator::{CreateOrganizator, Organizator, OrganizatorUpdate},
};
use macros::implementation;
use repository::organizator::{
    CreateOrganizator as CreateOrganizatorEntity, OrganizatorRepositoryDependency,
    OrganizatorUpdate as OrganizatorEntityUpdate,
};
use ulid::Ulid;
use utils::auth::{jwt, password_hashing::PasswordHasher};

implementation! {
    OrganizatorService {
        organizator_repository: OrganizatorRepositoryDependency,
        secret: String,
        password_hasher: PasswordHasher<'static>
    } as Implemented {
        register(
            &self,
            new: CreateOrganizator,
        ) -> (Organizator, String) {
            if self.organizator_repository.exists_by_username(&new.username).await? {
                Err(ServiceError::AlreadyExists("Organizator with provided username".into()))?
            }

            let organizator = self.organizator_repository
                .save(CreateOrganizatorEntity {
                    username: new.username.clone(),
                    password_hash: self.password_hasher.hash(&new.password)?,
                }).await?;

            let token = jwt::new("organizator", organizator.id.clone().into(), &self.secret);

            (organizator.into(), token)
        }

        login(
            &self,
            LoginRequest {
                login,
                password
            }: LoginRequest,
        ) -> (Organizator, String) {
            let organizator = self.organizator_repository
                .find_by_username(&login)
                .await?
                .ok_or(
                    ServiceError::NotFound("Organizator with provided username".into())
                )?;

            self.password_hasher
                .verify(&password, &organizator.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let token = jwt::new("organizator", organizator.id.clone().into(), &self.secret);

            (organizator.into(), token)
        }

        find_by_id(
            &self,
            id: Ulid,
        ) -> Option<Organizator> {
            self.organizator_repository
                .find_by_id(id.into())
                .await?
                .map(Organizator::from)
        }

        get_by_id(
            &self,
            id: Ulid,
        ) -> Organizator {
            self
                .find_by_id(id)
                .await?
                .ok_or(
                    ServiceError::NotFound("Organizator with provided id".into())
                )?
                .into()
        }

        update_by_id(
            &self,
            id: Ulid,
            update: OrganizatorUpdate,
        ) -> Organizator {
            self.get_by_id(id.clone()).await?;
            if update.username.is_some()
                && self.organizator_repository
                    .exists_by_username(update.username.as_ref().unwrap())
                    .await? {
                Err(ServiceError::AlreadyExists("Organizator with provided username".into()))?
            }

            self.organizator_repository
                .update_by_id(id.into(), OrganizatorEntityUpdate {
                    username: update.username,
                    password_hash: None,
                })
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
        ) -> (Organizator, String) {
            let organizator = self.organizator_repository
                .find_by_id(id.into())
                .await?
                .ok_or(
                    ServiceError::NotFound("Organizator with provided username".into())
                )?;

            PasswordHasher::new()
                .verify(&current_password, &organizator.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let organizator = self.organizator_repository
                .update_by_id(id.into(), OrganizatorEntityUpdate {
                    password_hash: Some(new_password),
                    ..Default::default()
                })
                .await?
                .unwrap();
            let token = jwt::new("organizator", organizator.id.clone().into(), &self.secret);

            (organizator.into(), token)
        }

        delete_by_id(
            &self,
            id: Ulid,
        ) -> () {
            self.get_by_id(id.clone()).await?;
            self.organizator_repository
                .delete_by_id(id.into())
                .await?;
        }
    }
}
