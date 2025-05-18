use std::collections::HashMap;

use entity::profile::{Profile, ProfileId, UpsertProfile};
use macros::{EntityId, implementation, surql_query};
use surrealdb::Value;
use tracing::instrument;
use utils::adapters::{MobcPool, SurrealPool};

use super::{ProfileRepository, ProfileRepositoryResult};
use crate::common::{ExtractValue as _, RepositoryError};

implementation! {
    ProfileRepository {
        pool: SurrealPool
    } as SurrealProfileRepository {
        #[instrument(skip_all, name = "ProfileRepository::upsert_by_id")]
        async fn upsert_by_id(&self, id: ProfileId, object: UpsertProfile) -> Profile {
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

        #[instrument(skip_all, name = "ProfileRepository::find_by_id")]
        async fn find_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.pool
                .get()
                .await?
                .select(id.record_id())
                .await?
        }

        #[instrument(skip_all, name = "ProfileRepository::exists_by_id")]
        async fn exists_by_id(&self, id: ProfileId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        #[instrument(skip_all, name = "ProfileRepository::delete_by_id")]
        async fn delete_by_id(&self, id: ProfileId) -> Option<Profile> {
            self.pool
                .get()
                .await?
                .delete(id.record_id())
                .await?
        }

        #[instrument(skip_all, name = "ProfileRepository::count_by_city")]
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
