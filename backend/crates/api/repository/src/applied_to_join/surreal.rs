use entity::{
    EntityId,
    applied_to_join::{
        AppliedToJoin, AppliedToJoinId, AppliedToJoinUpdate,
        CreateAppliedToJoin,
    },
    team::TeamId,
    user::UserId,
};
use macros::{implementation, surql_query};
use utils::adapters::{MobcPool, SurrealPool};

use super::{AppliedToJoinRepository, AppliedToJoinRepositoryResult};
use crate::common::RepositoryError;

#[implementation(
    result = AppliedToJoinRepositoryResult,
)]
pub mod repository {
    pub struct SurrealAppliedToJoinRepository {
        pool: SurrealPool,
    }

    impl AppliedToJoinRepository for SurrealAppliedToJoinRepository {
        async fn save(&self, new: CreateAppliedToJoin) -> AppliedToJoin {
            self.pool
                .get()
                .await?
                .create(new.get_id().record_id())
                .content(AppliedToJoin::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(
            &self,
            r#in: UserId,
            limit: u16,
            offset: u64,
        ) -> Vec<AppliedToJoin> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", AppliedToJoinId::TABLE))
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
            out: TeamId,
            limit: u16,
            offset: u64,
        ) -> Vec<AppliedToJoin> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", AppliedToJoinId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_out(&self, out: TeamId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        async fn find_by_in_and_out(
            &self,
            r#in: UserId,
            out: TeamId,
        ) -> Option<AppliedToJoin> {
            self.pool
                .get()
                .await?
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(
            &self,
            r#in: UserId,
            out: TeamId,
        ) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn update_by_in_and_out(
            &self,
            r#in: UserId,
            out: TeamId,
            update: AppliedToJoinUpdate,
        ) -> Option<AppliedToJoin> {
            self.pool
                .get()
                .await?
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        async fn delete_by_in_and_out(
            &self,
            r#in: UserId,
            out: TeamId,
        ) -> Option<AppliedToJoin> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
