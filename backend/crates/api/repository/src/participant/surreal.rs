use super::{CreateParticipant, Participant, ParticipantId, ParticipantUpdate};
use crate::common::adapters::surrealdb::SurrealDB;
use macros::implementation;
use std::sync::Arc;
use ulid::Ulid;

impl Into<Ulid> for ParticipantId {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into(self) -> Ulid {
        Ulid::from_string(&self.to_string()).unwrap()
    }
}

implementation! {
    ParticipantRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateParticipant) -> Participant {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .unwrap()
        }

        find_by_id(&self, id: ParticipantId) -> Option<Participant> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: ParticipantId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_email(&self, email: &str) -> Option<Participant> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($participant_table)
                            WHERE email = type::string($email)
                            LIMIT 1
                    "#
                )
                .bind(("participant_table", ParticipantId::TABLE))
                .bind(("email", email.to_string()))
                .await?
                .take(0)?
        }

        exists_by_email(&self, email: &str) -> bool {
            self.find_by_email(email).await?.is_some()
        }

        update_by_id(&self, id: ParticipantId, update: ParticipantUpdate) -> Option<Participant> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: ParticipantId) -> Option<Participant> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
