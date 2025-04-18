use std::sync::Arc;

use entity::{
    applied_to_join::{
        AppliedToJoin, AppliedToJoinId, AppliedToJoinUpdate,
        CreateAppliedToJoin,
    },
    team::TeamId,
    user::UserId,
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{AppliedToJoinRepository, AppliedToJoinRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    AppliedToJoinRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateAppliedToJoin) -> AppliedToJoin {
            self.db.0
                .create(new.get_id().record_id())
                .content(AppliedToJoin::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<AppliedToJoin> {
            self.db.0
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", AppliedToJoinId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        find_all_by_out(&self, out: TeamId, limit: u16, offset: u64) -> Vec<AppliedToJoin> {
            self.db.0
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", AppliedToJoinId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_out(&self, out: TeamId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        find_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<AppliedToJoin> {
            self.db.0
                .select(self.get_id(&r#in, &out))
                .await?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: TeamId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        update_by_in_and_out(&self, r#in: UserId, out: TeamId, update: AppliedToJoinUpdate) -> Option<AppliedToJoin> {
            self.db.0
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        delete_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<AppliedToJoin> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
