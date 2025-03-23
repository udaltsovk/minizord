use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    mentor::{CreateMentor, Mentor, MentorUpdate},
};
use macros::implementation;
use repository::mentor::{
    CreateMentor as CreateMentorEntity, Mentor as MentorEntity,
    MentorRepositoryDependency, MentorUpdate as MentorEntityUpdate,
};
use ulid::Ulid;
use utils::auth::{jwt, password_hashing::PasswordHasher};

use crate::common::ServiceError;

implementation! {
    MentorService {
        mentor_repository: MentorRepositoryDependency,
        secret: String,
        password_hasher: PasswordHasher<'static>
    } as Implemented {
        register(
            &self,
            new: CreateMentor,
        ) -> (Mentor, String) {
            if self.mentor_repository.exists_by_username(&new.username).await? {
                Err(ServiceError::AlreadyExists("Mentor with provided username".into()))?
            }

            let mentor = self.mentor_repository
                .save(
                    CreateMentorEntity {
                        username: new.username.clone(),
                        password_hash: self.password_hasher.hash(&new.password)?,
                        name: new.name.clone(),
                        surname: new.surname.clone(),
                        bio: new.bio.clone(),
                    }
                ).await?;

            let token = generate_jwt(&mentor, &self.secret);

            (mentor.into(), token)
        }

        login(
            &self,
            LoginRequest {
                login,
                password
            }: LoginRequest,
        ) -> (Mentor, String) {
            let mentor = self.mentor_repository
                .find_by_username(&login)
                .await?
                .ok_or(
                    ServiceError::NotFound("Mentor with provided username".into())
                )?;

            self.password_hasher
                .verify(&password, &mentor.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let token = generate_jwt(&mentor, &self.secret);

            (mentor.into(), token)
        }

        find_by_id(
            &self,
            id: Ulid,
        ) -> Option<Mentor> {
            self.mentor_repository
                .find_by_id(id.into())
                .await?
                .map(Mentor::from)
        }

        get_by_id(
            &self,
            id: Ulid,
        ) -> Mentor {
            self
                .find_by_id(id)
                .await?
                .ok_or(
                    ServiceError::NotFound("Mentor with provided id".into())
                )?
        }

        update_by_id(
            &self,
            id: Ulid,
            update: MentorUpdate,
        ) -> Mentor {
            self.get_by_id(id).await?;
            if update.username.is_some()
                && self.mentor_repository
                    .exists_by_username(update.username.as_ref().unwrap())
                    .await? {
                Err(ServiceError::AlreadyExists("Mentor with provided username".into()))?
            }

            self.mentor_repository
                .update_by_id(
                    id.into(),
                    MentorEntityUpdate {
                        username: update.username,
                        password_hash: None,
                        name: update.name,
                        surname: update.surname,
                        bio: update.bio,
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
        ) -> (Mentor, String) {
            let mentor = self.mentor_repository
                .find_by_id(id.into())
                .await?
                .ok_or(
                    ServiceError::NotFound("Mentor with provided username".into())
                )?;

            PasswordHasher::new()
                .verify(&current_password, &mentor.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let mentor = self.mentor_repository
                .update_by_id(
                    id.into(),
                    MentorEntityUpdate {
                        password_hash: Some(new_password),
                        ..Default::default()
                    }
                )
                .await?
                .unwrap();

            let token = generate_jwt(&mentor, &self.secret);

            (mentor.into(), token)
        }

        delete_by_id(
            &self,
            id: Ulid,
        ) -> () {
            self.get_by_id(id).await?;
            self.mentor_repository
                .delete_by_id(id.into())
                .await?;
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn generate_jwt(mentor: &MentorEntity, secret: &str) -> String {
    jwt::new("mentor", mentor.id.clone().into(), secret)
}
