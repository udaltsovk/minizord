use std::sync::Arc;

use entity::{
    team::TeamId,
    technology::TechnologyId,
    uses::{CreateUses, Uses, UsesId, UsesUpdate},
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{UsesRepository, UsesRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    UsesRepository {
        db: Arc<SurrealDB>
    } as SurrealUsesRepository {
        async fn save(&self, new: CreateUses) -> Uses {
            self.db.0
                .create(new.get_id().record_id())
                .content(Uses::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(&self, r#in: TeamId, limit: u16, offset: u64) -> Vec<Uses> {
            self.db.0
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

        async fn find_all_by_out(&self, out: TechnologyId, limit: u16, offset: u64) -> Vec<Uses> {
            self.db.0
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

        async fn find_by_in_and_out(&self, r#in: TeamId, out: TechnologyId) -> Option<Uses> {
            self.db.0
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(&self, r#in: TeamId, out: TechnologyId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn update_by_in_and_out(&self, r#in: TeamId, out: TechnologyId, update: UsesUpdate) -> Option<Uses> {
            self.db.0
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        async fn delete_by_in_and_out(&self, r#in: TeamId, out: TechnologyId) -> Option<Uses> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
