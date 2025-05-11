use std::sync::Arc;

use entity::{
    knows::{Knows, KnowsId, UpsertKnows},
    technology::TechnologyId,
    user::UserId,
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{KnowsRepository, KnowsRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    KnowsRepository {
        db: Arc<SurrealDB>
    } as SurrealKnowsRepository {
        async fn upsert_by_in_and_out(&self, r#in: UserId, out: TechnologyId, object: UpsertKnows) -> Knows {
            let result: Option<Knows> = self.db.0
                .query(surql_query!("relation/upsert_by_in_and_out"))
                .bind(("in", r#in))
                .bind(("id", object.get_id().record_id()))
                .bind(("out", out))
                .bind(("object", object))
                .await?
                .take(0)?;
            result.ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<Knows> {
            self.db.0
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", KnowsId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        async fn find_all_by_out(&self, out: TechnologyId, limit: u16, offset: u64) -> Vec<Knows> {
            self.db.0
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", KnowsId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_out(&self, out: TechnologyId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        async fn find_by_in_and_out(&self, r#in: UserId, out: TechnologyId) -> Option<Knows> {
            self.db.0
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(&self, r#in: UserId, out: TechnologyId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn delete_by_in_and_out(&self, r#in: UserId, out: TechnologyId) -> Option<Knows> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
