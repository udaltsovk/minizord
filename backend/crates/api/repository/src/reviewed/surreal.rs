use std::collections::HashMap;

use entity::{
    EntityId,
    reviewed::{Reviewed, ReviewedId, UpsertReviewed},
    user::UserId,
};
use macros::{implementation, surql_query};
use surrealdb::Value;
use utils::adapters::{MobcPool, SurrealPool};

use super::{ReviewedRepository, ReviewedRepositoryResult};
use crate::common::{ExtractValue as _, RepositoryError};

#[implementation(result = ReviewedRepositoryResult)]
pub mod repository {
    struct SurrealReviewedRepository {
        pool: SurrealPool,
    }

    impl ReviewedRepository for SurrealReviewedRepository {
        async fn upsert_by_in_and_out(
            &self,
            r#in: UserId,
            out: UserId,
            object: UpsertReviewed,
        ) -> Reviewed {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/upsert_by_in_and_out"))
                .bind(("in", r#in))
                .bind(("id", object.get_id().record_id()))
                .bind(("out", out))
                .bind(("object", object))
                .await?
                .take::<Option<Reviewed>>(0)?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(
            &self,
            r#in: UserId,
            limit: u16,
            offset: u64,
        ) -> Vec<Reviewed> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", ReviewedId::TABLE))
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
            out: UserId,
            limit: u16,
            offset: u64,
        ) -> Vec<Reviewed> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", ReviewedId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_out(&self, out: UserId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        async fn find_by_in_and_out(
            &self,
            r#in: UserId,
            out: UserId,
        ) -> Option<Reviewed> {
            self.pool
                .get()
                .await?
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(
            &self,
            r#in: UserId,
            out: UserId,
        ) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn delete_by_in_and_out(
            &self,
            r#in: UserId,
            out: UserId,
        ) -> Option<Reviewed> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }

        async fn count_by_score(&self) -> HashMap<u16, u32> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/count_by_field"))
                .bind(("table", ReviewedId::TABLE))
                .bind(("field", "score"))
                .await?
                .take::<Value>(0)?
                .extract()
        }
    }
}
