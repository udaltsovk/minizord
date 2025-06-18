use std::collections::HashMap;

use entity::{
    EntityId,
    profile::{Profile, ProfileId, UpsertProfile},
};
use macros::{implementation, surql_query};
use surrealdb::Value;
use utils::adapters::{MobcPool, SurrealPool};

use super::{ProfileRepository, ProfileRepositoryResult};
use crate::common::{ExtractValue as _, RepositoryError};

#[implementation(result = ProfileRepositoryResult)]
pub mod repository {
    struct SurrealProfileRepository {
        pool: SurrealPool,
    }

    impl ProfileRepository for SurrealProfileRepository {
        async fn upsert_by_id(
            &self,
            id: ProfileId,
            object: UpsertProfile,
        ) -> Profile {
            let entity = Profile::from((object, id));
            self.pool
                .get()
                .await?
                .query(surql_query!("table/upsert_by_id"))
                .bind(("id", entity.id.clone()))
                .bind(("object", entity))
                .await?
                .take::<Option<Profile>>(0)?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.pool.get().await?.select(id.record_id()).await?
        }

        async fn exists_by_id(&self, id: ProfileId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        async fn delete_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.pool.get().await?.delete(id.record_id()).await?
        }

        async fn count_by_city(&self) -> HashMap<String, u32> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/count_by_field"))
                .bind(("table", ProfileId::TABLE))
                .bind(("field", "city"))
                .await?
                .take::<Value>(0)?
                .extract()
        }
    }
}
