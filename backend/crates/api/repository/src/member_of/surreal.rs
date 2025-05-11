use std::sync::Arc;

use entity::{
    member_of::{CreateMemberOf, MemberOf, MemberOfId, MemberOfUpdate},
    team::TeamId,
    user::UserId,
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{MemberOfRepository, MemberOfRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    MemberOfRepository {
        db: Arc<SurrealDB>
    } as SurrealMemberOfRepository {
        async fn save(&self, new: CreateMemberOf) -> MemberOf {
            self.db.0
                .create(new.get_id().record_id())
                .content(MemberOf::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<MemberOf> {
            self.db.0
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

        async fn find_all_by_out(&self, out: TeamId, limit: u16, offset: u64) -> Vec<MemberOf> {
            self.db.0
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

        async fn find_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<MemberOf> {
            self.db.0
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(&self, r#in: UserId, out: TeamId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn update_by_in_and_out(&self, r#in: UserId, out: TeamId, update: MemberOfUpdate) -> Option<MemberOf> {
            self.db.0
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        async fn delete_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<MemberOf> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
