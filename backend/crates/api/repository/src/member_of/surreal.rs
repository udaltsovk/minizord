use std::sync::Arc;

use entity::{
    member_of::{CreateMemberOf, MemberOf, MemberOfUpdate},
    team::TeamId,
    user::UserId,
};
use macros::implementation;
use utils::adapters::SurrealDB;

use super::{MemberOfRepository, MemberOfRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    MemberOfRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateMemberOf) -> MemberOf {
            self.db.0
                .create(new.get_id(self.table()))
                .content(new.into_entity())
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<MemberOf> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE in = type::record($in)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", self.table()))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        find_all_by_out(&self, out: TeamId, limit: u16, offset: u64) -> Vec<MemberOf> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE out = type::record($out)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", self.table()))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_out(&self, out: TeamId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        find_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<MemberOf> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE in = type::record($in) 
                                && out = type::record($out)
                            LIMIT 1
                    "#
                )
                .bind(("table", self.table()))
                .bind(("in", r#in))
                .bind(("out", out))
                .await?
                .take(0)?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: TeamId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        update_by_in_and_out(&self, r#in: UserId, out: TeamId, update: MemberOfUpdate) -> Option<MemberOf> {
            self.db.0
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        delete_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<MemberOf> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
