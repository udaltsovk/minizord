use entity::{
    EntityId,
    member_of::{CreateMemberOf, MemberOf, MemberOfId, MemberOfUpdate},
    team::TeamId,
    user::UserId,
};
use macros::{implementation, surql_query};
use utils::adapters::{MobcPool, SurrealPool};

use super::{MemberOfRepository, MemberOfRepositoryResult};
use crate::common::RepositoryError;

#[implementation(result = MemberOfRepositoryResult)]
pub mod repository {
    struct SurrealMemberOfRepository {
        pool: SurrealPool,
    }

    impl MemberOfRepository for SurrealMemberOfRepository {
        async fn save(&self, new: CreateMemberOf) -> MemberOf {
            self.pool
                .get()
                .await?
                .create(new.get_id().record_id())
                .content(MemberOf::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(
            &self,
            r#in: UserId,
            limit: u16,
            offset: u64,
        ) -> Vec<MemberOf> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", MemberOfId::TABLE))
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
        ) -> Vec<MemberOf> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", MemberOfId::TABLE))
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
        ) -> Option<MemberOf> {
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
            update: MemberOfUpdate,
        ) -> Option<MemberOf> {
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
        ) -> Option<MemberOf> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
