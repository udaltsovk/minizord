use entity::{
    EntityId,
    team::TeamId,
    technology::TechnologyId,
    uses::{CreateUses, Uses, UsesId, UsesUpdate},
};
use macros::{implementation, surql_query};
use utils::adapters::{MobcPool, SurrealPool};

use super::{UsesRepository, UsesRepositoryResult};
use crate::common::RepositoryError;

#[implementation(result = UsesRepositoryResult)]
pub mod repository {
    struct SurrealUsesRepository {
        pool: SurrealPool,
    }

    impl UsesRepository for SurrealUsesRepository {
        async fn save(&self, new: CreateUses) -> Uses {
            self.pool
                .get()
                .await?
                .create(new.get_id().record_id())
                .content(Uses::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(
            &self,
            r#in: TeamId,
            limit: u16,
            offset: u64,
        ) -> Vec<Uses> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", UsesId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_in(&self, r#in: TeamId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        async fn find_all_by_out(
            &self,
            out: TechnologyId,
            limit: u16,
            offset: u64,
        ) -> Vec<Uses> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", UsesId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_out(&self, out: TechnologyId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        async fn find_by_in_and_out(
            &self,
            r#in: TeamId,
            out: TechnologyId,
        ) -> Option<Uses> {
            self.pool
                .get()
                .await?
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(
            &self,
            r#in: TeamId,
            out: TechnologyId,
        ) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn update_by_in_and_out(
            &self,
            r#in: TeamId,
            out: TechnologyId,
            update: UsesUpdate,
        ) -> Option<Uses> {
            self.pool
                .get()
                .await?
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        async fn delete_by_in_and_out(
            &self,
            r#in: TeamId,
            out: TechnologyId,
        ) -> Option<Uses> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
