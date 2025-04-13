use std::sync::Arc;

use entity::profile::{Profile, ProfileId, UpsertProfile};
use macros::{EntityId, implementation};
use utils::adapters::SurrealDB;

use super::{ProfileRepository, ProfileRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    ProfileRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        upsert_by_id(&self, id: ProfileId, object: UpsertProfile) -> Profile {
            let entity = object.into_entity(id);
            let result: Option<Profile> = self.db.0
                .query(include_str!("../../db/surreal/queries/table/upsert_by_id.surql"))
                .bind(("id", entity.id.clone()))
                .bind(("object", entity))
                .await?
                .take(0)?;
            result.ok_or(RepositoryError::FailedToSaveObject)?
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
