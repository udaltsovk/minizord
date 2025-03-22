use crate::common::ServiceError;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    participant::{CreateParticipant, Participant, ParticipantUpdate},
};
use macros::implementation;
use repository::participant::{
    CreateParticipant as CreateParticipantEntity, ParticipantRepositoryDependency,
    ParticipantUpdate as ParticipantEntityUpdate,
};
use ulid::Ulid;
use utils::auth::{jwt, password_hashing::PasswordHasher};

implementation! {
    ParticipantService {
        participant_repository: ParticipantRepositoryDependency,
        secret: String,
        password_hasher: PasswordHasher<'static>
    } as Implemented {
        register(
            &self,
            new: CreateParticipant,
        ) -> (Participant, String) {
            if self.participant_repository.exists_by_username(&new.username).await? {
                Err(ServiceError::AlreadyExists("Participant with provided username".into()))?
            }

            let participant = self.participant_repository
                .save(CreateParticipantEntity {
                    username: new.username.clone(),
                    password_hash: self.password_hasher.hash(&new.password)?,
                    name: new.name.clone(),
                    surname: new.surname.clone(),
                    bio: new.bio.clone(),
                    portfolio_urls: new.portfolio_urls,
                }).await?;

            let token = jwt::new("participant", participant.id.clone().into(), &self.secret);

            (participant.into(), token)
        }

        login(
            &self,
            LoginRequest {
                login,
                password
            }: LoginRequest,
        ) -> (Participant, String) {
            let participant = self.participant_repository
                .find_by_username(&login)
                .await?
                .ok_or(
                    ServiceError::NotFound("Participant with provided username".into())
                )?;

            self.password_hasher
                .verify(&password, &participant.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let token = jwt::new("participant", participant.id.clone().into(), &self.secret);

            (participant.into(), token)
        }

        find_by_id(
            &self,
            id: Ulid,
        ) -> Option<Participant> {
            self.participant_repository
                .find_by_id(id.into())
                .await?
                .map(Participant::from)
        }

        get_by_id(
            &self,
            id: Ulid,
        ) -> Participant {
            self
                .find_by_id(id)
                .await?
                .ok_or(
                    ServiceError::NotFound("Participant with provided id".into())
                )?
                .into()
        }

        update_by_id(
            &self,
            id: Ulid,
            update: ParticipantUpdate,
        ) -> Participant {
            self.get_by_id(id.clone()).await?;
            if update.username.is_some()
                && self.participant_repository
                    .exists_by_username(update.username.as_ref().unwrap())
                    .await? {
                Err(ServiceError::AlreadyExists("Participant with provided username".into()))?
            }

            self.participant_repository
                .update_by_id(id.into(), ParticipantEntityUpdate {
                    username: update.username,
                    password_hash: None,
                    name: update.name,
                    surname: update.surname,
                    bio: update.bio,
                    portfolio_urls: update.portfolio_urls,
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
        ) -> (Participant, String) {
            let participant = self.participant_repository
                .find_by_id(id.into())
                .await?
                .ok_or(
                    ServiceError::NotFound("Participant with provided username".into())
                )?;

            PasswordHasher::new()
                .verify(&current_password, &participant.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let participant = self.participant_repository
                .update_by_id(id.into(), ParticipantEntityUpdate {
                    password_hash: Some(new_password),
                    ..Default::default()
                })
                .await?
                .unwrap();
            let token = jwt::new("participant", participant.id.clone().into(), &self.secret);

            (participant.into(), token)
        }

        delete_by_id(
            &self,
            id: Ulid,
        ) -> () {
            self.get_by_id(id.clone()).await?;
            self.participant_repository
                .delete_by_id(id.into())
                .await?;
        }
    }
}
