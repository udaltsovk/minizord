use std::sync::Arc;

use macros::implementation;

use super::{CreateHasExperienceAs, HasExperienceAs, HasExperienceAsUpdate};
use crate::{
    common::adapters::surrealdb::SurrealDB, specialization::SpecializationId,
    user::UserId,
};

implementation! {
    HasExperienceAsRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateHasExperienceAs) -> HasExperienceAs {
            self.db.0
                .create(new.get_id(Self::TABLE))
                .content(new.into_entity())
                .await?
                .unwrap()
        }

        find_all_by_in(&self, r#in: UserId, limit: u64, offset: u64) -> Vec<HasExperienceAs> {
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

        find_all_by_out(&self, out: SpecializationId, limit: u64, offset: u64) -> Vec<HasExperienceAs> {
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

        update_by_in_and_out(&self, r#in: UserId, out: SpecializationId, update: HasExperienceAsUpdate) -> Option<HasExperienceAs> {
            self.db.0
                .update(Self::get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        delete_by_in_and_out(&self, r#in: UserId, out: SpecializationId) -> Option<HasExperienceAs> {
            self.db.0
                .delete(Self::get_id(&r#in, &out))
                .await?
        }
    }
}
