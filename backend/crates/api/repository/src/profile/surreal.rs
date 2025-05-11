use std::sync::Arc;

use entity::profile::{Profile, ProfileId, UpsertProfile};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{ProfileRepository, ProfileRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    ProfileRepository {
        db: Arc<SurrealDB>
    } as SurrealProfileRepository {
        async fn upsert_by_id(&self, id: ProfileId, object: UpsertProfile) -> Profile {
            let entity = Profile::from((object, id));
            let result: Option<Profile> = self.db.0
                .query(surql_query!("table/upsert_by_id"))
                .bind(("id", entity.id.clone()))
                .bind(("object", entity))
                .await?
                .take(0)?;
            result.ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        async fn exists_by_id(&self, id: ProfileId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        async fn delete_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
