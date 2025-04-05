use std::sync::Arc;

use macros::implementation;
use utils::adapters::SurrealDB;

use super::{HasExperienceAs, UpsertHasExperienceAs};
use crate::{
    common::RepositoryError, specialization::SpecializationId, user::UserId,
};

implementation! {
    HasExperienceAsRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        upsert_by_in_and_out(&self, r#in: UserId, out: SpecializationId, object: UpsertHasExperienceAs) -> HasExperienceAs {
            let result: Option<HasExperienceAs> = self.db.0
                .query(
                    r#"
                        RELATE ONLY (type::record($in))->(type::record($id))->(type::record($out))
                            CONTENT <object>$object
                    "#
                )
                .bind(("in", r#in))
                .bind(("id", object.get_id(Self::TABLE)))
                .bind(("out", out))
                .bind(("object", object))
                .await?
                .take(0)?;
            result.ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_all_by_in(&self, r#in: UserId, limit: u64, offset: u64) -> Vec<HasExperienceAs> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE in = type::record($in)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", Self::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        find_all_by_out(&self, out: SpecializationId, limit: u64, offset: u64) -> Vec<HasExperienceAs> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE out = type::record($out)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", Self::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_out(&self, out: SpecializationId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        find_by_in_and_out(&self, r#in: UserId, out: SpecializationId) -> Option<HasExperienceAs> {
            self.db.0
                .select(Self::get_id(&r#in, &out))
                .await?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: SpecializationId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        delete_by_in_and_out(&self, r#in: UserId, out: SpecializationId) -> Option<HasExperienceAs> {
            self.db.0
                .delete(Self::get_id(&r#in, &out))
                .await?
        }
    }
}
