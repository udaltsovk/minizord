use std::sync::Arc;

use entity::{
    reviewed::{Reviewed, UpsertReviewed},
    user::UserId,
};
use macros::implementation;
use utils::adapters::SurrealDB;

use super::{ReviewedRepository, ReviewedRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    ReviewedRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        upsert_by_in_and_out(&self, r#in: UserId, out: UserId, object: UpsertReviewed) -> Reviewed {
            let result: Option<Reviewed> = self.db.0
                .query(
                    r#"
                        RELATE ONLY (type::record($in))->(type::record($id))->(type::record($out))
                            CONTENT <object>$object
                    "#
                )
                .bind(("in", r#in))
                .bind(("id", object.get_id(self.table())))
                .bind(("out", out))
                .bind(("object", object))
                .await?
                .take(0)?;
            result.ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<Reviewed> {
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

        find_all_by_out(&self, out: UserId, limit: u16, offset: u64) -> Vec<Reviewed> {
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

        exists_by_out(&self, out: UserId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        find_by_in_and_out(&self, r#in: UserId, out: UserId) -> Option<Reviewed> {
            self.db.0
                .select(self.get_id(&r#in, &out))
                .await?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: UserId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        delete_by_in_and_out(&self, r#in: UserId, out: UserId) -> Option<Reviewed> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
