use entity::{
    knows::{Knows, KnowsId, UpsertKnows},
    technology::TechnologyId,
    user::UserId,
};
use macros::{EntityId, implementation, surql_query};
use tracing::instrument;
use utils::adapters::{MobcPool, SurrealPool};

use super::{KnowsRepository, KnowsRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    KnowsRepository {
        pool: SurrealPool
    } as SurrealKnowsRepository {
        #[instrument(skip_all, name = "KnowsRepository::upsert_by_in_and_out")]
        async fn upsert_by_in_and_out(&self, r#in: UserId, out: TechnologyId, object: UpsertKnows) -> Knows {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/upsert_by_in_and_out"))
                .bind(("in", r#in))
                .bind(("id", object.get_id().record_id()))
                .bind(("out", out))
                .bind(("object", object))
                .await?
                .take::<Option<Knows>>(0)?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        #[instrument(skip_all, name = "KnowsRepository::find_all_by_in")]
        async fn find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<Knows> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", KnowsId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        #[instrument(skip_all, name = "KnowsRepository::exists_by_in")]
        async fn exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        #[instrument(skip_all, name = "KnowsRepository::find_all_by_out")]
        async fn find_all_by_out(&self, out: TechnologyId, limit: u16, offset: u64) -> Vec<Knows> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", KnowsId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        #[instrument(skip_all, name = "KnowsRepository::exists_by_out")]
        async fn exists_by_out(&self, out: TechnologyId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        #[instrument(skip_all, name = "KnowsRepository::find_by_in_and_out")]
        async fn find_by_in_and_out(&self, r#in: UserId, out: TechnologyId) -> Option<Knows> {
            self.pool
                .get()
                .await?
                .select(self.get_id(&r#in, &out))
                .await?
        }

        #[instrument(skip_all, name = "KnowsRepository::exists_by_in_and_out")]
        async fn exists_by_in_and_out(&self, r#in: UserId, out: TechnologyId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        #[instrument(skip_all, name = "KnowsRepository::delete_by_in_and_out")]
        async fn delete_by_in_and_out(&self, r#in: UserId, out: TechnologyId) -> Option<Knows> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
