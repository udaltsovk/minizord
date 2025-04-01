use std::sync::Arc;

use macros::{RepositoryId, implementation};
use ulid::Ulid;

use super::{CreateProfile, Profile, ProfileId, ProfileUpdate};
use crate::common::adapters::surrealdb::SurrealDB;

impl From<ProfileId> for Ulid {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(id: ProfileId) -> Self {
        Self::from_string(&id.to_string()).unwrap()
    }
}

implementation! {
    ProfileRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateProfile) -> Profile {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .unwrap()
        }

        find_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: ProfileId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        update_by_id(&self, id: ProfileId, update: ProfileUpdate) -> Option<Profile> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
