use entity::{
    EntityId,
    has_experience_as::{
        HasExperienceAs, HasExperienceAsId, UpsertHasExperienceAs,
    },
    specialization::SpecializationId,
    user::UserId,
};
use macros::{implementation, surql_query};
use utils::adapters::{MobcPool, SurrealPool};

use super::{HasExperienceAsRepository, HasExperienceAsRepositoryResult};
use crate::common::RepositoryError;

#[implementation(result = HasExperienceAsRepositoryResult)]
pub mod repository {
    struct SurrealHasExperienceAsRepository {
        pool: SurrealPool,
    }

    impl HasExperienceAsRepository for SurrealHasExperienceAsRepository {
        async fn upsert_by_in_and_out(
            &self,
            r#in: UserId,
            out: SpecializationId,
            object: UpsertHasExperienceAs,
        ) -> HasExperienceAs {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/upsert_by_in_and_out"))
                .bind(("in", r#in))
                .bind(("id", object.get_id().record_id()))
                .bind(("out", out))
                .bind(("object", object))
                .await?
                .take::<Option<HasExperienceAs>>(0)?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(
            &self,
            r#in: UserId,
            limit: u16,
            offset: u64,
        ) -> Vec<HasExperienceAs> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", HasExperienceAsId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        async fn find_all_by_out(
            &self,
            out: SpecializationId,
            limit: u16,
            offset: u64,
        ) -> Vec<HasExperienceAs> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", HasExperienceAsId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_out(&self, out: SpecializationId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        async fn find_by_in_and_out(
            &self,
            r#in: UserId,
            out: SpecializationId,
        ) -> Option<HasExperienceAs> {
            self.pool
                .get()
                .await?
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(
            &self,
            r#in: UserId,
            out: SpecializationId,
        ) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn delete_by_in_and_out(
            &self,
            r#in: UserId,
            out: SpecializationId,
        ) -> Option<HasExperienceAs> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
