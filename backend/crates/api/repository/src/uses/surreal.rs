use entity::{
    team::TeamId,
    technology::TechnologyId,
    uses::{CreateUses, Uses, UsesId, UsesUpdate},
};
use macros::{EntityId, implementation, surql_query};
use tracing::instrument;
use utils::adapters::{MobcPool, SurrealPool};

use super::{UsesRepository, UsesRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    UsesRepository {
        pool: SurrealPool
    } as SurrealUsesRepository {
        #[instrument(skip_all, name = "UsesRepository::save")]
        async fn save(&self, new: CreateUses) -> Uses {
            self.pool
                .get()
                .await?
                .create(new.get_id().record_id())
                .content(Uses::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        #[instrument(skip_all, name = "UsesRepository::find_all_by_in")]
        async fn find_all_by_in(&self, r#in: TeamId, limit: u16, offset: u64) -> Vec<Uses> {
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

        #[instrument(skip_all, name = "UsesRepository::exists_by_in")]
        async fn exists_by_in(&self, r#in: TeamId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        #[instrument(skip_all, name = "UsesRepository::find_all_by_out")]
        async fn find_all_by_out(&self, out: TechnologyId, limit: u16, offset: u64) -> Vec<Uses> {
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

        #[instrument(skip_all, name = "UsesRepository::exists_by_out")]
        async fn exists_by_out(&self, out: TechnologyId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        #[instrument(skip_all, name = "UsesRepository::find_by_in_and_out")]
        async fn find_by_in_and_out(&self, r#in: TeamId, out: TechnologyId) -> Option<Uses> {
            self.pool
                .get()
                .await?
                .select(self.get_id(&r#in, &out))
                .await?
        }

        #[instrument(skip_all, name = "UsesRepository::exists_by_in_and_out")]
        async fn exists_by_in_and_out(&self, r#in: TeamId, out: TechnologyId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        #[instrument(skip_all, name = "UsesRepository::update_by_in_and_out")]
        async fn update_by_in_and_out(&self, r#in: TeamId, out: TechnologyId, update: UsesUpdate) -> Option<Uses> {
            self.pool
                .get()
                .await?
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        #[instrument(skip_all, name = "UsesRepository::delete_by_in_and_out")]
        async fn delete_by_in_and_out(&self, r#in: TeamId, out: TechnologyId) -> Option<Uses> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
