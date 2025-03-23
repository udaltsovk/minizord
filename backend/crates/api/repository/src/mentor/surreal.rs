use std::sync::Arc;

use macros::implementation;
use ulid::Ulid;

use super::{CreateMentor, Mentor, MentorId, MentorUpdate};
use crate::common::adapters::surrealdb::SurrealDB;

impl From<MentorId> for Ulid {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(id: MentorId) -> Self {
        Self::from_string(&id.to_string()).unwrap()
    }
}

implementation! {
    MentorRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateMentor) -> Mentor {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .unwrap()
        }

        find_by_id(&self, id: MentorId) -> Option<Mentor> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: MentorId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_username(&self, username: &str) -> Option<Mentor> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($mentor_table)
                            WHERE username = type::string($username)
                            LIMIT 1
                    "#
                )
                .bind(("mentor_table", MentorId::TABLE))
                .bind(("username", username.to_string()))
                .await?
                .take(0)?
        }

        exists_by_username(&self, username: &str) -> bool {
            self.find_by_username(username).await?.is_some()
        }

        update_by_id(&self, id: MentorId, update: MentorUpdate) -> Option<Mentor> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: MentorId) -> Option<Mentor> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
