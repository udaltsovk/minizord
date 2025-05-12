use entity::{
    member_of::{CreateMemberOf, MemberOf, MemberOfId, MemberOfUpdate},
    team::TeamId,
    user::UserId,
};
use macros::{EntityId, implementation, surql_query};
use tracing::instrument;
use utils::adapters::SurrealDB;

use super::{MemberOfRepository, MemberOfRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    MemberOfRepository {
        db: SurrealDB
    } as SurrealMemberOfRepository {
        #[instrument(skip_all, name = "MemberOfRepository::save")]
        async fn save(&self, new: CreateMemberOf) -> MemberOf {
            self.db
                .create(new.get_id().record_id())
                .content(MemberOf::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        #[instrument(skip_all, name = "MemberOfRepository::find_all_by_in")]
        async fn find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<MemberOf> {
            self.db
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", MemberOfId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        #[instrument(skip_all, name = "MemberOfRepository::exists_by_in")]
        async fn exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        #[instrument(skip_all, name = "MemberOfRepository::find_all_by_out")]
        async fn find_all_by_out(&self, out: TeamId, limit: u16, offset: u64) -> Vec<MemberOf> {
            self.db
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", MemberOfId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        #[instrument(skip_all, name = "MemberOfRepository::exists_by_out")]
        async fn exists_by_out(&self, out: TeamId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        #[instrument(skip_all, name = "MemberOfRepository::find_by_in_and_out")]
        async fn find_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<MemberOf> {
            self.db
                .select(self.get_id(&r#in, &out))
                .await?
        }

        #[instrument(skip_all, name = "MemberOfRepository::exists_by_in_and_out")]
        async fn exists_by_in_and_out(&self, r#in: UserId, out: TeamId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        #[instrument(skip_all, name = "MemberOfRepository::update_by_in_and_out")]
        async fn update_by_in_and_out(&self, r#in: UserId, out: TeamId, update: MemberOfUpdate) -> Option<MemberOf> {
            self.db
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        #[instrument(skip_all, name = "MemberOfRepository::delete_by_in_and_out")]
        async fn delete_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<MemberOf> {
            self.db
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
