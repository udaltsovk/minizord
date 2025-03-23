use crate::common::ServiceError;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    participant::{CreateParticipant, Participant, ParticipantUpdate},
};
use macros::implementation;
use repository::participant::{
    CreateParticipant as CreateParticipantEntity,
    Participant as ParticipantEntity, ParticipantRepositoryDependency,
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
            if self.participant_repository.exists_by_email(&new.email).await? {
                Err(ServiceError::AlreadyExists("Participant with provided email".into()))?
            }

            let participant = self.participant_repository
                .save(
                    CreateParticipantEntity {
                        email: new.email.clone(),
                        password_hash: self.password_hasher.hash(&new.password)?,
                        name: new.name.clone(),
                        surname: new.surname.clone(),
                        bio: new.bio.clone(),
                        portfolio_urls: new.portfolio_urls,
                    }
                ).await?;

            let token = generate_jwt(&participant, &self.secret);

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
                .find_by_email(&login)
                .await?
                .ok_or(
                    ServiceError::NotFound("Participant with provided email".into())
                )?;

            self.password_hasher
                .verify(&password, &participant.password_hash)
                .map_err(|_| ServiceError::InvalidPassword)?;

            let token = generate_jwt(&participant, &self.secret);

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

            self.participant_repository
                .update_by_id(
                    id.into(),
                    ParticipantEntityUpdate {
                        password_hash: None,
                        name: update.name,
                        surname: update.surname,
                        bio: update.bio,
                        portfolio_urls: update.portfolio_urls,
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
                .update_by_id(
                    id.into(),
                    ParticipantEntityUpdate {
                        password_hash: Some(new_password),
                        ..Default::default()
                    }
                )
                .await?
                .unwrap();
            let token = generate_jwt(&participant, &self.secret);

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

#[tracing::instrument(skip_all, level = "trace")]
fn generate_jwt(participant: &ParticipantEntity, secret: &str) -> String {
    jwt::new("participant", participant.id.clone().into(), secret)
}
