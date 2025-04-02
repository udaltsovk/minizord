use std::sync::Arc;

use macros::{RepositoryId, implementation};
use ulid::Ulid;

use super::{Profile, ProfileId, UpsertProfile};
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
        upsert_by_id(&self, id: ProfileId, object: UpsertProfile) -> Profile {
            let entity = object.into_entity(id);
            let result: Option<Profile> = self.db.0
                .query(r#"
                    UPSERT ONLY type::record($id) CONTENT <object>$object
                "#)
                .bind(("id", entity.id.clone()))
                .bind(("object", entity))
                .await?
                .take(0)?;
            result.unwrap()
        }

        find_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: ProfileId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        delete_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
