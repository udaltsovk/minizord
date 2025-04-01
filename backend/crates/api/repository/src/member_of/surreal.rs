use std::sync::Arc;

use macros::implementation;

use super::{CreateMemberOf, MemberOf, MemberOfUpdate};
use crate::{
    common::adapters::surrealdb::SurrealDB, team::TeamId, user::UserId,
};

implementation! {
    MemberOfRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateMemberOf) -> MemberOf {
            self.db.0
                .create(new.get_id(Self::TABLE))
                .content(new.into_entity())
                .await?
                .unwrap()
        }

        find_all_by_in(&self, r#in: UserId, limit: u64, offset: u64) -> Vec<MemberOf> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE in = type::string($in)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", Self::TABLE))
                .bind(("in", r#in.to_string()))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        find_all_by_out(&self, out: TeamId, limit: u64, offset: u64) -> Vec<MemberOf> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE out = type::string($out)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", Self::TABLE))
                .bind(("out", out.to_string()))
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
                            WHERE in = type::string($in) 
                                && out = type::string($out)
                            LIMIT 1
                    "#
                )
                .bind(("table", Self::TABLE))
                .bind(("in", r#in.to_string()))
                .bind(("out", out.to_string()))
                .await?
                .take(0)?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: TeamId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        update_by_in_and_out(&self, r#in: UserId, out: TeamId, update: MemberOfUpdate) -> Option<MemberOf> {
            self.db.0
                .update(Self::get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        delete_by_in_and_out(&self, r#in: UserId, out: TeamId) -> Option<MemberOf> {
            self.db.0
                .delete(Self::get_id(&r#in, &out))
                .await?
        }
    }
}
